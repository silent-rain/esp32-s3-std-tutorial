use esp_idf_svc::{
    hal::{gpio::*, peripherals::Peripherals, task::block_on},
    sys::link_patches,
};

fn main() -> anyhow::Result<()> {
    link_patches();

    let peripherals = Peripherals::take()?;

    let mut button = PinDriver::input(peripherals.pins.gpio4)?;

    let mut led = PinDriver::output(peripherals.pins.gpio5)?;

    button.set_pull(Pull::Up)?;

    block_on(async {
        loop {
            button.wait_for_high().await?;

            led.set_high()?;

            button.wait_for_low().await?;

            led.set_low()?;
        }
    })
}
