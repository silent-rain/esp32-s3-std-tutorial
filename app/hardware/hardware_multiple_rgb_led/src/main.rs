use neopixel::{NeoPixel, Rgb};

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

    let channel = peripherals.rmt.channel0;
    let mut neopixel = NeoPixel::new(peripherals.pins.gpio48, channel, 1)?;

    loop {
        // 红色
        neopixel.set_color(0, Rgb::new(255, 0, 0))?;
        neopixel.show()?;
        // 延迟
        FreeRtos::delay_ms(1000);
        // 绿色
        neopixel.set_color(0, Rgb::new(0, 255, 0))?;
        neopixel.show()?;
        // 延迟
        FreeRtos::delay_ms(1000);
        // 蓝色
        neopixel.set_color_and_show(0, Rgb::new(0, 0, 255))?;
        // 延迟
        FreeRtos::delay_ms(1000);

        log::warn!("this is warn!");
    }
}
