use esp_idf_svc::{
    hal::{
        adc::{attenuation, config::Config, AdcChannelDriver, AdcDriver},
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

    // 用于配置ADC（模拟数字转换器）
    let mut adc = AdcDriver::new(peripherals.adc1, &Config::new().calibration(true))?;

    // 配置引脚为模拟读取，可以根据需要调节ADC的输入电压范围
    // 本例中使用了11db的衰减，将输入电压范围设置为大约0-3.6V
    // 电位器传感器
    let mut potentiometer: AdcChannelDriver<{ attenuation::DB_11 }, _> =
        AdcChannelDriver::new(peripherals.pins.gpio4)?;
    // 反射式红外传感器
    let mut reflective_infrared: AdcChannelDriver<{ attenuation::DB_11 }, _> =
        AdcChannelDriver::new(peripherals.pins.gpio5)?;
    // 热敏传感器
    let mut thermistor: AdcChannelDriver<{ attenuation::DB_11 }, _> =
        AdcChannelDriver::new(peripherals.pins.gpio6)?;
    // 光敏传感器
    let mut photosensor: AdcChannelDriver<{ attenuation::DB_11 }, _> =
        AdcChannelDriver::new(peripherals.pins.gpio7)?;

    log::warn!("loop");
    loop {
        let potentiometer_value = adc.read(&mut potentiometer)?;
        let reflective_infrared_value = adc.read(&mut reflective_infrared)?;
        let thermistor_value = adc.read(&mut thermistor)?;
        let photosensor_value = adc.read(&mut photosensor)?;

        log::info!(
            "ADC:
        potentiometer sensor: {potentiometer_value}
        reflective infrared sensor: {reflective_infrared_value}
        thermistor sensor: {thermistor_value}
        photo sensor: {photosensor_value}",
        );

        FreeRtos::delay_ms(500);
    }
}
