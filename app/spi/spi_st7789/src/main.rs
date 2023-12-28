use display_interface_spi::SPIInterfaceNoCS;
use esp_idf_svc::{
    hal::{
        delay::{Ets, FreeRtos},
        gpio::PinDriver,
        peripherals::Peripherals,
        prelude::FromValueType,
        spi::{
            config::MODE_3, Operation, SpiConfig, SpiDeviceDriver, SpiDriver, SpiDriverConfig, SPI2,
        },
    },
    log::EspLogger,
    sys::link_patches,
};
use mipidsi::{Builder, Orientation};

fn main() -> anyhow::Result<()> {
    link_patches();
    // Bind the log crate to the ESP Logging facilities
    EspLogger::initialize_default();
    // 设置日志级别
    log::set_max_level(log::LevelFilter::Info);

    // Get the peripherals
    let peripherals = Peripherals::take()?;

    let spi = peripherals.spi2;

    let rst = PinDriver::output(peripherals.pins.gpio3)?;
    let dc = PinDriver::output(peripherals.pins.gpio4)?;
    let mut backlight = PinDriver::output(peripherals.pins.gpio5)?;
    let sclk = peripherals.pins.gpio6;
    let sda = peripherals.pins.gpio7;
    let sdi = peripherals.pins.gpio8;
    let cs = peripherals.pins.gpio10;

    let mut delay = Ets;

    // configuring the spi interface, note that in order for the ST7789 to work, the data_mode needs to be set to MODE_3
    let config = SpiConfig::new().baudrate(26.MHz().into()).data_mode(MODE_3);

    let device = SpiDeviceDriver::new_single(
        spi,
        sclk,
        sda,
        Some(sdi),
        Some(cs),
        &SpiDriverConfig::new(),
        &config,
    )?;

    // display interface abstraction from SPI and DC
    let di = SPIInterfaceNoCS::new(device, dc);

    // create driver
    let mut display = Builder::st7789(di)
        .with_display_size(240, 240)
        // set default orientation
        .with_orientation(Orientation::Portrait(false))
        // initialize
        .init(&mut delay, Some(rst))
        .unwrap();

    // turn on the backlight
    backlight.set_high()?;
    let raw_image_data = ImageRawLE::new(include_bytes!("../examples/assets/ferris.raw"), 86);
    let ferris = Image::new(&raw_image_data, Point::new(0, 0));

    // draw image on black background
    display.clear(Rgb565::BLACK).unwrap();
    ferris.draw(&mut display).unwrap();

    println!("Image printed!");

    loop {
        thread::sleep(Duration::from_millis(1000));
    }
}
