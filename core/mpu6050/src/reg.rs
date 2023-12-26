//! 寄存器版本实现

use super::conf::*;
pub use super::AccelGyroData;

use esp_idf_hal::{
    gpio::{InputOutput, InputPin, Output, OutputPin, Pin, PinDriver},
    sys::EspError,
};

const DEFAULT_SLAVE_ADDR: u8 = 0xD0;

/// MPU6050 芯片
pub struct Mpu6050<'d, SclPin, SdaPin>
where
    SclPin: Pin + OutputPin,
    SdaPin: Pin + InputPin + OutputPin,
{
    scl: PinDriver<'d, SclPin, Output>,
    sda: PinDriver<'d, SdaPin, InputOutput>,
}

impl<'d, SclPin, SdaPin> Mpu6050<'d, SclPin, SdaPin>
where
    SclPin: Pin + OutputPin,
    SdaPin: Pin + InputPin + OutputPin,
{
    fn i2c_w_scl(&mut self, bit_value: u8) -> Result<(), EspError> {
        if bit_value == 0 {
            self.scl.set_low()?;
        } else {
            self.scl.set_high()?;
        }
        Ok(())
    }

    fn i2c_w_sda(&mut self, bit_value: u8) -> Result<(), EspError> {
        if bit_value == 0 {
            self.sda.set_low()?;
        } else {
            self.sda.set_high()?;
        }
        Ok(())
    }

    fn i2c_r_sda(&mut self) -> Result<u8, EspError> {
        let bit_value = self.sda.is_high();
        if bit_value {
            Ok(1)
        } else {
            Ok(0)
        }
    }

    /// 产生 I2C 协议起始信号
    fn i2c_start(&mut self) -> Result<(), EspError> {
        self.i2c_w_sda(1)?;
        self.i2c_w_scl(1)?;
        self.i2c_w_sda(0)?;
        self.i2c_w_scl(0)?;
        Ok(())
    }

    /// 产生 I2C 协议结束信号
    fn i2c_stop(&mut self) -> Result<(), EspError> {
        self.i2c_w_sda(0)?;
        self.i2c_w_scl(1)?;
        self.i2c_w_sda(1)?;
        Ok(())
    }

    /// 发送八位数据（不包含应答）
    fn i2c_send_byte(&mut self, byte: u8) -> Result<(), EspError> {
        for i in 0..8 {
            self.i2c_w_sda(byte & (0x80 >> i))?;
            self.i2c_w_scl(1)?;
            self.i2c_w_scl(0)?;
        }
        Ok(())
    }

    /// 读取八位数据（不包含应答）
    fn i2c_receive_byte(&mut self) -> Result<u8, EspError> {
        self.i2c_w_sda(1)?;

        let mut byte = 0x00;
        for i in 0..8 {
            self.i2c_w_scl(1)?;
            if self.i2c_r_sda()? == 1 {
                byte |= 0x80 >> i;
            }
            self.i2c_w_scl(0)?;
        }
        Ok(byte)
    }

    /// 发送应答信号
    fn i2c_send_ack(&mut self, ack_bit: u8) -> Result<(), EspError> {
        self.i2c_w_sda(ack_bit)?;
        self.i2c_w_scl(1)?;
        self.i2c_w_scl(0)?;
        Ok(())
    }

    /// 接收应答信号
    fn i2c_receive_ack(&mut self) -> Result<u8, EspError> {
        self.i2c_w_sda(1)?;
        self.i2c_w_scl(1)?;
        let ack_bit = self.i2c_r_sda()?;
        self.i2c_w_scl(0)?;
        Ok(ack_bit)
    }

    /// I2C 初始化
    fn init_i2c(&mut self) -> Result<(), EspError> {
        self.i2c_w_scl(1)?;
        self.i2c_w_sda(1)?;
        Ok(())
    }
}

impl<'d, SclPin, SdaPin> Mpu6050<'d, SclPin, SdaPin>
where
    SclPin: Pin + OutputPin,
    SdaPin: Pin + InputPin + OutputPin,
{
    pub fn new(scl: SclPin, sda: SdaPin) -> Result<Self, EspError> {
        let scl_driver = PinDriver::output_od(scl)?;
        let sda_driver = PinDriver::input_output(sda)?;
        let mut mpu = Mpu6050 {
            scl: scl_driver,
            sda: sda_driver,
        };

        // I2C 初始化
        mpu.init_i2c()?;

        // MPU6050 初始化
        mpu.init_mpu6050()?;
        Ok(mpu)
    }

    /// MPU6050 初始化
    pub fn init_mpu6050(&mut self) -> Result<(), EspError> {
        // 解除休眠状态
        self.write_reg(MPU6050_PWR_MGMT_1, 0x01)?;
        self.write_reg(MPU6050_PWR_MGMT_2, 0x00)?;
        // 陀螺仪采样率，典型值：0x07(125Hz)
        self.write_reg(MPU6050_SMPLRT_DIV, 0x09)?;
        // 低通滤波频率，典型值：0x06(5Hz)
        self.write_reg(MPU6050_CONFIG, 0x06)?;
        // 陀螺仪自检及测量范围，典型值：0x18(不自检，2000deg/s)
        self.write_reg(MPU6050_GYRO_CONFIG, 0x18)?;
        // 加速计自检、测量范围及高通滤波频率，典型值：0x01(不自检，2G，5Hz)
        self.write_reg(MPU6050_ACCEL_CONFIG, 0x18)?;

        Ok(())
    }

    /// MPU6050 写寄存器函数
    /// reg_address：寄存器地址
    /// data：待写入寄存器值
    pub fn write_reg(&mut self, reg_address: u8, data: u8) -> Result<(), EspError> {
        // 发送起始信号
        self.i2c_start()?;

        // 发送设备地址
        self.i2c_send_byte(DEFAULT_SLAVE_ADDR)?;
        self.i2c_receive_ack()?;

        // 发送寄存器地址
        self.i2c_send_byte(reg_address)?;
        self.i2c_receive_ack()?;

        // 写数据到寄存器
        self.i2c_send_byte(data)?;
        self.i2c_receive_ack()?;

        self.i2c_stop()?;

        Ok(())
    }

    /// 读取寄存器
    pub fn read_reg(&mut self, reg_address: u8) -> Result<i16, EspError> {
        // 发送起始信号
        self.i2c_start()?;

        // 发送设备地址
        self.i2c_send_byte(DEFAULT_SLAVE_ADDR)?;
        self.i2c_receive_ack()?;

        // 发送寄存器地址
        self.i2c_send_byte(reg_address)?;
        self.i2c_receive_ack()?;

        // 发送重复起始信号
        self.i2c_start()?;
        // 发送读模式设备地址
        self.i2c_send_byte(DEFAULT_SLAVE_ADDR | 0x01)?;
        self.i2c_receive_ack()?;

        // 读寄存器数据
        let data = self.i2c_receive_byte()?;
        // 非应答信号
        self.i2c_send_ack(1)?;

        self.i2c_stop()?;

        Ok(data as i16)
    }

    /// 获取 MPU6050 ID
    pub fn get_id(&mut self) -> Result<u8, EspError> {
        let data = self.read_reg(MPU6050_WHO_AM_I)?;

        Ok(data as u8)
    }

    /// 基本数据读取
    /// 连续读两个寄存器并合成 16 位数据
    pub fn get_data(&mut self) -> Result<AccelGyroData, EspError> {
        let mut data = AccelGyroData::default();

        let data_h = self.read_reg(MPU6050_ACCEL_XOUT_H)?;
        let data_l = self.read_reg(MPU6050_ACCEL_XOUT_L)?;
        data.acc_x = (data_h << 8) | data_l;

        let data_h = self.read_reg(MPU6050_ACCEL_YOUT_H)?;
        let data_l = self.read_reg(MPU6050_ACCEL_YOUT_L)?;
        data.acc_y = (data_h << 8) | data_l;

        let data_h = self.read_reg(MPU6050_ACCEL_ZOUT_H)?;
        let data_l = self.read_reg(MPU6050_ACCEL_ZOUT_L)?;
        data.acc_z = (data_h << 8) | data_l;

        let data_h = self.read_reg(MPU6050_GYRO_XOUT_H)?;
        let data_l = self.read_reg(MPU6050_GYRO_XOUT_L)?;
        data.gyro_x = (data_h << 8) | data_l;

        let data_h = self.read_reg(MPU6050_GYRO_YOUT_H)?;
        let data_l = self.read_reg(MPU6050_GYRO_YOUT_L)?;
        data.gyro_y = (data_h << 8) | data_l;

        let data_h = self.read_reg(MPU6050_GYRO_ZOUT_H)?;
        let data_l = self.read_reg(MPU6050_GYRO_ZOUT_L)?;
        data.gyro_z = (data_h << 8) | data_l;

        Ok(data)
    }
}
