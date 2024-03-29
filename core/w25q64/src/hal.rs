use esp_idf_hal::{
    gpio::AnyIOPin,
    peripheral::Peripheral,
    prelude::FromValueType,
    spi::{
        config::{BitOrder, MODE_0},
        SpiAnyPins, SpiConfig, SpiDeviceDriver, SpiDriver, SpiDriverConfig,
    },
    sys::EspError,
};

use super::conf::*;

pub struct W25Q64<'d> {
    spi: SpiDeviceDriver<'d, SpiDriver<'d>>,
}

impl<'d> W25Q64<'d> {
    pub fn new<SPI: SpiAnyPins>(
        spi: impl Peripheral<P = SPI> + 'd,
        cs: AnyIOPin,
        sck: AnyIOPin,
        mosi: AnyIOPin,
        miso: AnyIOPin,
    ) -> Result<Self, EspError> {
        // 创建一个Spi实例
        let spi_driver = SpiDriver::new(spi, sck, miso, Some(mosi), &SpiDriverConfig::new())?;

        let config = SpiConfig::new()
            .baudrate(1.MHz().into())
            // 先行位，选择高位先行
            .bit_order(BitOrder::MsbFirst)
            .allow_pre_post_delays(true)
            // 配置 SPI 的极性、相位
            .data_mode(MODE_0)
            // 设置默认电平, SS默认高电平
            .cs_active_high();
        let spi_device_driver = SpiDeviceDriver::new(spi_driver, Some(cs), &config)?;

        let w25q = W25Q64 {
            spi: spi_device_driver,
        };
        Ok(w25q)
    }

    /// 启用写入功能
    pub fn write_enable(&mut self) -> Result<(), EspError> {
        self.spi.write(&[W25Q64_WRITE_ENABLE])
    }

    /// 禁用写入功能
    pub fn write_disable(&mut self) -> Result<(), EspError> {
        self.spi.write(&[W25Q64_WRITE_DISABLE])
    }

    /// 读取芯片的JEDEC设备ID
    /// 使用Spi实例和片选引脚来发送和接收命令和数据
    pub fn read_jedec_device_id(&mut self) -> Result<(u8, u8, u8), EspError> {
        let mut buf = [W25Q64_JEDEC_DEVICE_ID, 0, 0, 0];
        self.spi.transfer_in_place(&mut buf)?;

        let manufacturer_id = buf[1];
        let memory_type = buf[2];
        let capacity = buf[3];
        Ok((manufacturer_id, memory_type, capacity))
    }

    /// 读取芯片的制造商和设备ID
    ///
    /// 使用Spi实例和片选引脚来发送和接收命令和数据
    /// 0xEF16: 代表W25Q64芯片
    pub fn read_manufacturer_device_id(&mut self) -> Result<(u16, u16), EspError> {
        let mut buf = [0; 7];
        buf[0] = W25Q64_MANUFACTURER_DEVICE_ID;

        // 发送读取制造商和设备ID的命令
        self.spi.transfer_in_place(&mut buf)?;

        let manufacturer_id = buf[4] as u16;
        let device_id = (buf[5] as u16) << 8 | buf[6] as u16;
        Ok((manufacturer_id, device_id))
    }

    /// 读取状态寄存器1
    pub fn read_status_register_1(&mut self) -> Result<u8, EspError> {
        let mut buf = [W25Q64_READ_STATUS_REGISTER_1, 0];
        self.spi.transfer_in_place(&mut buf)?;

        self.spi.read(&mut buf)?;
        Ok(buf[1])
    }

    /// 检查是否有写保护标志
    pub fn check_write_protect(&mut self) -> Result<bool, EspError> {
        let status = self.read_status_register_1()?;
        let srp0 = status & 0x80;
        let srp1 = status & 0x04;
        if srp0 == 0 && srp1 == 0 {
            // 没有写保护
            Ok(false)
        } else {
            // 有写保护
            Ok(true)
        }
    }

    /// 定义一个辅助函数，用于等待W25Q64芯片空闲
    pub fn wait_for_idle(&mut self) -> Result<(), EspError> {
        // 发送读状态寄存器1命令
        let mut buf = [W25Q64_READ_STATUS_REGISTER_1, 0x00];

        // 给定超时计数时间
        let mut timeout = 100000;

        // 循环等待忙标志位
        loop {
            // 接收状态寄存器1的值
            self.spi.transfer_in_place(&mut buf)?;
            if buf[1] & 0x01 == 0 {
                // 检查状态寄存器1的最低位，如果为0表示空闲，否则表示忙碌
                break;
            }
            timeout -= 1;
            if timeout == 0 {
                break;
            }
        }
        Ok(())
    }

    /// 页编程, 写入数据
    /// page_address: 设定页地址
    /// data: 要写入的数据
    pub fn page_program(&mut self, page_address: u32, data: &[u8]) -> Result<(), EspError> {
        assert!(data.len() <= 256); // A page is 256 bytes

        self.write_enable()?;

        let cmd = [
            W25Q64_PAGE_PROGRAM,        // 页编程的指令
            (page_address >> 16) as u8, // 地址23~16位
            (page_address >> 8) as u8,  // 地址15~8位
            page_address as u8,         // 地址7~0位
        ];

        self.spi.write(&cmd)?;
        self.spi.write(data)?;

        // 等待W25Q64芯片空闲
        self.wait_for_idle()?;
        Ok(())
    }

    /// 擦除地址所在的扇区
    pub fn sector_erase(&mut self, address: u32) -> Result<(), EspError> {
        self.write_enable()?;

        let cmd = [
            W25Q64_SECTOR_ERASE_4KB, // 扇区擦除的指令
            (address >> 16) as u8,   // 地址23~16位
            (address >> 8) as u8,    // 地址15~8位
            address as u8,           // 地址7~0位
        ];
        self.spi.write(&cmd)?;

        self.wait_for_idle()?;
        Ok(())
    }

    /// 擦除闪存芯片上的所有扇区
    /// 这是一项非常昂贵的手术
    pub fn erase_chip(&mut self) -> Result<(), EspError> {
        self.write_enable()?;

        let cmd = [W25Q64_CHIP_ERASE];
        self.spi.write(&cmd)?;

        self.wait_for_idle()?;
        Ok(())
    }

    /// 读取数据
    /// read_address: 目标地址
    /// data: 用于存放数据
    pub fn read_data(&mut self, read_address: u32, data: &mut [u8]) -> Result<(), EspError> {
        // 使用fill方法来赋值为虚拟字节
        // data.fill(W25Q64_DUMMY_BYTE);

        let cmd = [
            W25Q64_READ_DATA,           // 读取数据的指令
            (read_address >> 16) as u8, // 地址23~16位
            (read_address >> 8) as u8,  // 地址15~8位
            read_address as u8,         // 地址7~0位
        ];
        self.spi.write(&cmd)?;
        self.spi.read(data)?;

        Ok(())
    }
}
