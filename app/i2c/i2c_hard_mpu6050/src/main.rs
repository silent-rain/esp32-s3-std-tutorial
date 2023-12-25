use mpu6050::hal::Mpu6050;

use esp_idf_svc::{
    hal::{delay::FreeRtos, peripherals::Peripherals},
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

    let mut mpu = Mpu6050::new(i2c, sda, scl)?;
    mpu.wake_up()?;
    let id = mpu.get_id()?;
    log::info!("MPU6050 ID {id}");

    log::info!("loop");
    loop {
        let data = mpu.get_data()?;

        // 打印读取到的数据
        println!("Accel: ({}, {}, {})", data.acc_x, data.acc_y, data.acc_z);
        println!("Gyro: ({}, {}, {})", data.gyro_x, data.gyro_y, data.gyro_z);
        FreeRtos::delay_ms(1000);
    }
}
