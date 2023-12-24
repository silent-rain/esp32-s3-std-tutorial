use anyhow::Context;
use esp_idf_svc::{
    hal::{delay::FreeRtos, peripherals::Peripherals},
    log::EspLogger,
    sys::link_patches,
};
use pcnt_encoder::Encoder;

fn main() -> anyhow::Result<()> {
    link_patches();
    EspLogger::initialize_default();
    log::set_max_level(log::LevelFilter::Info);

    println!("setup pins");
    let peripherals = Peripherals::take().context("failed to take Peripherals")?;
    let mut pin_a = peripherals.pins.gpio4;
    let mut pin_b = peripherals.pins.gpio5;
    println!("setup encoder");
    let encoder = Encoder::new(peripherals.pcnt0, &mut pin_a, &mut pin_b)?;

    let mut last_value = 0_i32;
    loop {
        let value = encoder.get_value()?;
        if value != last_value {
            println!("value: {value}");
            last_value = value;
        }
        FreeRtos::delay_ms(100u32);
    }
}
