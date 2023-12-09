#![allow(clippy::empty_loop)]
use oled::OLED;

use esp_idf_svc::{
    hal::{delay::FreeRtos, peripherals::Peripherals},
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

    let pins = peripherals.pins;
    let mut oled = OLED::new(pins.gpio4, pins.gpio5)?;

    oled.show_string(1, 1, "tiele");
    oled.show_string(2, 1, "this is test.");

    log::warn!("loop");
    loop {
        FreeRtos::delay_ms(1000);
    }
}
