//! ADC oneshot example, reading a value form a pin and printing it on the terminal
//! requires ESP-IDF v5.0 or newer
#![allow(clippy::empty_loop)]

use esp_idf_svc::{
    hal::{
        adc::{
            attenuation::DB_11,
            oneshot::{config::AdcChannelConfig, AdcChannelDriver, AdcDriver},
        },
        delay::FreeRtos,
        peripherals::Peripherals,
    },
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

    // configuring pin to analog read, you can regulate the adc input voltage range depending on your need
    // for this example we use the attenuation of 11db which sets the input voltage range to around 0-3.6V
    let config = AdcChannelConfig {
        attenuation: DB_11,
        calibration: true,
        ..Default::default()
    };

    // 配置 ADC 引脚
    let adc = AdcDriver::new(peripherals.adc1)?;

    // 配置遥控杆输入引脚
    let mut adc_x_pin = AdcChannelDriver::new(&adc, peripherals.pins.gpio4, &config)?;
    let mut adc_y_pin = AdcChannelDriver::new(&adc, peripherals.pins.gpio5, &config)?;

    log::warn!("loop");
    loop {
        let value = adc.read(&mut adc_x_pin)?;
        log::info!("X Pin ADC value: {}", value);
        let value = adc.read(&mut adc_y_pin)?;
        log::info!("Y Pin ADC value: {}", value);
        FreeRtos::delay_ms(500);
    }
}
