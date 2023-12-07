#![allow(clippy::empty_loop)]

use esp_idf_svc::{
    hal::{delay::FreeRtos, gpio::PinDriver, peripherals::Peripherals},
    log::EspLogger,
    sys::link_patches,
};

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    link_patches();

    // Bind the log crate to the ESP Logging facilities
    EspLogger::initialize_default();

    // 设置日志级别
    log::set_max_level(log::LevelFilter::Info);

    // Get the peripherals
    let peripherals = Peripherals::take()?;

    let mut led = PinDriver::output(peripherals.pins.gpio4)?;
    FreeRtos::delay_ms(1000);

    log::trace!("this is trace!");
    log::debug!("this is debug!");
    log::info!("this is info!");
    log::warn!("this is warn!");
    log::error!("this is error!");
    loop {}
}
