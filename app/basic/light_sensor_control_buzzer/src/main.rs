use esp_idf_svc::{
    hal::{
        delay::FreeRtos,
        gpio::{PinDriver, Pull},
        peripherals::Peripherals,
    },
    log::EspLogger,
    sys::link_patches,
};

fn main() -> anyhow::Result<()> {
    link_patches();

    EspLogger::initialize_default();
    log::set_max_level(log::LevelFilter::Info);

    // Get the peripherals
    let peripherals = Peripherals::take()?;

    let mut buzzer = PinDriver::output(peripherals.pins.gpio4)?;

    let mut light_sensor = PinDriver::input(peripherals.pins.gpio5)?;

    // 停止蜂鸣
    buzzer.set_high()?;
    light_sensor.set_pull(Pull::Up)?;

    log::info!("loop");
    loop {
        FreeRtos::delay_ms(10);

        if light_sensor.is_high() {
            log::info!("is_high");
            buzzer.set_low()?;
        } else {
            // 停止蜂鸣
            buzzer.set_high()?;
        }
    }
}
