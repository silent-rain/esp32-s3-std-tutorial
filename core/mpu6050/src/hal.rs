//! HAL 库版本实现

use super::conf::*;
pub use super::AccelGyroData;

use esp_idf_hal::{
    delay::BLOCK,
    gpio::{InputPin, OutputPin},
    i2c::{I2c, I2cConfig, I2cDriver},
    peripheral::Peripheral,
    prelude::FromValueType,
    sys::EspError,
};

/// MPU6050 芯片
pub struct Mpu6050<'d> {
    i2c: I2cDriver<'d>,
}

impl<'d> Mpu6050<'d> {
    ///  初始化 MPU6050
    pub fn new<I2C: I2c>(
        i2c: impl Peripheral<P = I2C> + 'd,
        sda: impl Peripheral<P = impl InputPin + OutputPin> + 'd,
        scl: impl Peripheral<P = impl InputPin + OutputPin> + 'd,
    ) -> Result<Self, EspError> {
        let config = I2cConfig::new().baudrate(10.kHz().into());
        let i2c = I2cDriver::new(i2c, sda, scl, &config)?;

        let mpu = Mpu6050 { i2c };
        Ok(mpu)
    }

    /// 唤醒 MPU6050
    pub fn wake_up(&mut self) -> Result<(), EspError> {
        // 解除休眠状态
        self.i2c
            .write(DEFAULT_SLAVE_ADDR, &[MPU6050_PWR_MGMT_1, 0x01], BLOCK)?;
        self.i2c
            .write(DEFAULT_SLAVE_ADDR, &[MPU6050_PWR_MGMT_2, 0x00], BLOCK)?;
        // 陀螺仪采样率，典型值：0x07(125Hz)
        self.i2c
            .write(DEFAULT_SLAVE_ADDR, &[MPU6050_SMPLRT_DIV, 0x09], BLOCK)?;

        // 低通滤波频率，典型值：0x06(5Hz)
        self.i2c
            .write(DEFAULT_SLAVE_ADDR, &[MPU6050_CONFIG, 0x06], BLOCK)?;
        // 陀螺仪自检及测量范围，典型值：0x18(不自检，2000deg/s)
        self.i2c
            .write(DEFAULT_SLAVE_ADDR, &[MPU6050_GYRO_CONFIG, 0x18], BLOCK)?;
        // 加速计自检、测量范围及高通滤波频率，典型值：0x01(不自检，2G，5Hz
        self.i2c
            .write(DEFAULT_SLAVE_ADDR, &[MPU6050_ACCEL_CONFIG, 0x18], BLOCK)?;

        Ok(())
    }

    /// 获取 MPU6050 ID
    pub fn get_id(&mut self) -> Result<u8, EspError> {
        // 创建一个缓冲区用于存储数据
        let mut buffer: [u8; 14] = [0; 14];

        // 检查mpu6050的设备ID是否正确
        self.i2c.write_read(
            DEFAULT_SLAVE_ADDR,
            &[MPU6050_WHO_AM_I],
            &mut buffer[0..1],
            BLOCK,
        )?;

        // assert_eq!(buffer[0], DEFAULT_SLAVE_ADDR);
        Ok(buffer[0])
    }

    /// 获取 MPU6050 数据
    /// 读取加速度和角速度数据
    pub fn get_data(&mut self) -> Result<AccelGyroData, EspError> {
        // 创建一个缓冲区用于存储数据
        let mut buffer: [u8; 14] = [0; 14];

        // 从mpu6050中读取14个字节的数据，包括加速度和角速度
        self.i2c.write_read(
            DEFAULT_SLAVE_ADDR,
            &[MPU6050_ACCEL_XOUT_H],
            &mut buffer,
            BLOCK,
        )?;

        // 将数据转换为有符号的16位整数
        let acc_x = (buffer[0] as i16) << 8 | buffer[1] as i16;
        let acc_y = (buffer[2] as i16) << 8 | buffer[3] as i16;
        let acc_z = (buffer[4] as i16) << 8 | buffer[5] as i16;
        let gyro_x = (buffer[8] as i16) << 8 | buffer[9] as i16;
        let gyro_y = (buffer[10] as i16) << 8 | buffer[11] as i16;
        let gyro_z = (buffer[12] as i16) << 8 | buffer[13] as i16;

        Ok(AccelGyroData {
            acc_x,
            acc_y,
            acc_z,
            gyro_x,
            gyro_y,
            gyro_z,
        })
    }
}
