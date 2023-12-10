use esp_idf_svc::{
    hal::{delay::FreeRtos, gpio::*, peripherals::Peripherals},
    sys::link_patches,
};

fn main() -> anyhow::Result<()> {
    link_patches();

    let peripherals = Peripherals::take()?;

    let mut button = PinDriver::input(peripherals.pins.gpio4)?;

    let mut led = PinDriver::output(peripherals.pins.gpio5)?;

    button.set_pull(Pull::Up)?;

    log::info!("loop");
    loop {
        // we are using thread::sleep here to make sure the watchdog isn't triggered
        FreeRtos::delay_ms(10);

        if button.is_high() {
            log::info!("is_high");
            led.toggle()?;
        }
    }
}
