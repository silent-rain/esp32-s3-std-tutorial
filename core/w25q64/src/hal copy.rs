use esp_idf_hal::{
    gpio::{AnyIOPin, Output, PinDriver},
    peripheral::Peripheral,
    prelude::FromValueType,
    spi::{SpiAnyPins, SpiConfig, SpiDeviceDriver, SpiDriver, SpiDriverConfig},
    sys::EspError,
};

use super::conf::*;

pub struct W25Q64<'d> {
    spi: SpiDeviceDriver<'d, SpiDriver<'d>>,
    cs: PinDriver<'d, AnyIOPin, Output>,
}

impl<'d> W25Q64<'d> {
    pub fn new<SPI: SpiAnyPins>(
        spi: impl Peripheral<P = SPI> + 'd,
        ss: Option<AnyIOPin>,
        cs: AnyIOPin,
        sck: AnyIOPin,
        mosi: AnyIOPin,
        miso: AnyIOPin,
    ) -> Result<Self, EspError> {
        // 创建一个Spi实例
        let spi_driver = SpiDriver::new(spi, sck, miso, Some(mosi), &SpiDriverConfig::new())?;

        let config = SpiConfig::new().baudrate(26.MHz().into());
        let spi_device_driver = SpiDeviceDriver::new(spi_driver, ss, &config)?;

        let cs_driver = PinDriver::output(cs)?;

        let mut w25q = W25Q64 {
            spi: spi_device_driver,
            cs: cs_driver,
        };
        // 设置默认电平, SS默认高电平
        w25q.spi_w_ss(1)?;

        Ok(w25q)
    }

    /// SPI起始
    /// 此函数需要用户实现内容，当BitValue为0时，需要置SS为低电平，当BitValue为1时，需要置SS为高电平
    pub fn spi_w_ss(&mut self, bit_value: u8) -> Result<(), EspError> {
        if bit_value == 0 {
            // 拉低片选信号，开始通信
            self.cs.set_low()?;
        } else {
            // 拉高片选信号，结束通信
            self.cs.set_high()?;
        }

        Ok(())
    }

    /// SPI起始
    /// 拉低SS，开始时序
    pub fn spi_start(&mut self) -> Result<(), EspError> {
        self.spi_w_ss(0)
    }

    /// SPI终止
    /// 拉高SS，终止时序
    pub fn spi_stop(&mut self) -> Result<(), EspError> {
        self.spi_w_ss(1)
    }

    /// 写入数据
    pub fn spi_write(&mut self, words: &[u8]) -> Result<(), EspError> {
        self.spi_start()?;
        self.spi.write(words)?;
        self.spi_stop()?;

        Ok(())
    }

    /// 写入并返回数据
    pub fn spi_transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), EspError> {
        self.spi_start()?;
        self.spi.transfer(read, write)?;
        self.spi_stop()?;
        Ok(())
    }

    /// 启用写入功能
    pub fn write_enable(&mut self) -> Result<(), EspError> {
        self.spi_write(&[W25Q64_WRITE_ENABLE])?;
        Ok(())
    }

    /// 禁用写入功能
    pub fn write_disable(&mut self) -> Result<(), EspError> {
        self.spi_write(&[W25Q64_WRITE_DISABLE])?;
        Ok(())
    }

    /// 读取芯片的JEDEC设备ID
    /// 使用Spi实例和片选引脚来发送和接收命令和数据
    pub fn read_jedec_device_id(&mut self) -> Result<(u8, u8, u8), EspError> {
        let mut rx_buf = [0; 4];
        let mut tx_buf = [0; 4];
        tx_buf[0] = W25Q64_JEDEC_DEVICE_ID;

        self.spi_transfer(&mut rx_buf, &tx_buf)?;

        let manufacturer_id = rx_buf[1];
        let memory_type = rx_buf[2];
        let capacity = rx_buf[3];
        Ok((manufacturer_id, memory_type, capacity))
    }

    /// 读取芯片的制造商和设备ID
    ///
    /// 使用Spi实例和片选引脚来发送和接收命令和数据
    /// 0xEF16: 代表W25Q64芯片
    pub fn read_manufacturer_device_id(&mut self) -> Result<(u16, u16), EspError> {
        let mut rx_buf = [0; 7];
        let mut tx_buf = [0; 7];
        tx_buf[0] = W25Q64_MANUFACTURER_DEVICE_ID;

        // 发送读取制造商和设备ID的命令
        self.spi_transfer(&mut rx_buf, &tx_buf)?;

        let manufacturer_id = rx_buf[4] as u16;
        let device_id = (rx_buf[5] as u16) << 8 | rx_buf[6] as u16;
        Ok((manufacturer_id, device_id))
    }

    /// 读取状态寄存器1
    pub fn read_status_register_1(&mut self) -> Result<u8, EspError> {
        let mut rx_buf = [0; 2];
        let tx_buf = [W25Q64_READ_STATUS_REGISTER_1, 0];
        self.spi_transfer(&mut rx_buf, &tx_buf)?;

        self.spi.read(&mut rx_buf)?;
        Ok(rx_buf[1])
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
    pub fn _wait_for_idle(&mut self) -> Result<(), EspError> {
        // 发送读状态寄存器1命令
        let mut rx_buf = [0; 2];
        let tx_buf = [W25Q64_READ_STATUS_REGISTER_1, 0x00];

        // 给定超时计数时间
        let mut timeout = 100000;

        // 循环等待忙标志位
        loop {
            // 接收状态寄存器1的值
            self.spi_transfer(&mut rx_buf, &tx_buf)?;
            println!("buffer: {:#?}", rx_buf);
            if rx_buf[1] & 0x01 == 0 {
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

    /// 定义一个辅助函数，用于等待W25Q64芯片空闲
    pub fn wait_for_idle(&mut self) -> Result<(), EspError> {
        // let mut timeout = 100000;
        // // 空闲时退出
        // while self.spi.is_busy() {
        //     timeout -= 1;
        //     if timeout == 0 {
        //         break;
        //     }
        // }
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

        self.spi_start()?;
        self.spi.write(&cmd)?;
        self.spi.write(data)?;
        self.spi_stop()?;

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
        self.spi_write(&cmd)?;

        self.wait_for_idle()?;
        Ok(())
    }

    /// 擦除闪存芯片上的所有扇区
    /// 这是一项非常昂贵的手术
    pub fn erase_chip(&mut self) -> Result<(), EspError> {
        self.write_enable()?;

        let cmd = [W25Q64_CHIP_ERASE];
        self.spi_write(&cmd)?;

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
        self.spi_start()?;
        self.spi.write(&cmd)?;
        self.spi.read(data)?;
        self.spi_stop()?;

        println!("data {:#?}", data);
        Ok(())
    }
}
