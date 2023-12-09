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

    // 配置 ADC
    let adc = AdcDriver::new(peripherals.adc1)?;

    // 配置遥控杆输入引脚
    // 上下油门
    let mut throttle_adc = AdcChannelDriver::new(&adc, peripherals.pins.gpio4, &config)?;
    // 左右偏航
    let mut yaw_adc = AdcChannelDriver::new(&adc, peripherals.pins.gpio5, &config)?;

    // 上下俯仰
    let mut pitch_adc = AdcChannelDriver::new(&adc, peripherals.pins.gpio6, &config)?;
    // 左右横滚
    let mut roll_adc = AdcChannelDriver::new(&adc, peripherals.pins.gpio7, &config)?;

    log::warn!("loop");
    loop {
        let throttle_value = adc.read(&mut throttle_adc)?;
        log::info!("Throttle ADC value: {}", throttle_value);
        let yaw_value = adc.read(&mut yaw_adc)?;
        log::info!("Yaw ADC value: {}", yaw_value);

        let pitch_value = adc.read(&mut pitch_adc)?;
        log::info!("Pitch ADC value: {}", pitch_value);
        let roll_value = adc.read(&mut roll_adc)?;
        log::info!("Roll ADC value: {}", roll_value);
        FreeRtos::delay_ms(500);
    }
}
