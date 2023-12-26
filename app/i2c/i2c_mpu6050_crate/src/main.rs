use esp32s3_mpu6050::{device::WHOAMI, Mpu6050};

use esp_idf_svc::{
    hal::{
        delay::FreeRtos,
        i2c::{I2cConfig, I2cDriver},
        peripherals::Peripherals,
        prelude::FromValueType,
    },
    log::EspLogger,
    sys::link_patches,
};

fn main() -> anyhow::Result<()> {
    link_patches();
    // Bind the log crate to the ESP Logging facilities
    EspLogger::initialize_default();
    // 设置日志级别
    log::set_max_level(log::LevelFilter::Info);

    // Get the peripherals
    let peripherals = Peripherals::take()?;

    let i2c = peripherals.i2c0;
    let sda = peripherals.pins.gpio5;
    let scl = peripherals.pins.gpio6;
    let config = I2cConfig::new().baudrate(10.kHz().into());
    let i2c = I2cDriver::new(i2c, sda, scl, &config)?;

    let mut mpu = Mpu6050::new(i2c);
    // 初始化mpu6050
    mpu.init()?;

    // 获取 mpu6050 ID
    let address = mpu.read_byte(WHOAMI).unwrap();
    log::info!("MPU6050 address {address}");

    log::info!("loop");
    loop {
        // 获取温度数据，单位为摄氏度
        let temp = mpu.get_temp().unwrap();
        log::info!("Temperature: {}°C", temp);

        // 获取加速度数据，单位为g
        let acc = mpu.get_acc().unwrap();
        log::info!("Accel: ({}, {}, {})", acc.x, acc.y, acc.z);

        // 获取角速度数据，单位为弧度每秒
        let gyro = mpu.get_gyro().unwrap();
        log::info!("Gyro: ({}, {}, {})", gyro.x, gyro.y, gyro.z);

        FreeRtos::delay_ms(500);
    }
}
