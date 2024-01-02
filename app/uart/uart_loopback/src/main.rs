use esp_idf_svc::{
    hal::{
        delay::BLOCK,
        gpio,
        peripherals::Peripherals,
        uart::{
            config::{DataBits, StopBits},
            UartConfig, UartDriver,
        },
        units::Hertz,
    },
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

    let tx = peripherals.pins.gpio12;
    let rx = peripherals.pins.gpio13;

    println!("Starting UART loopback test");
    // let config = config::Config::new().baudrate(Hertz(115_200));
    let config = UartConfig::new()
        .baudrate(Hertz(115_200))
        .stop_bits(StopBits::STOP2)
        .data_bits(DataBits::DataBits8)
        .parity_odd();
    let uart = UartDriver::new(
        peripherals.uart1,
        tx,
        rx,
        Option::<gpio::Gpio0>::None,
        Option::<gpio::Gpio1>::None,
        &config,
    )?;

    loop {
        uart.write(&[0xaa])?;

        let mut buf = [0_u8; 1];
        uart.read(&mut buf, BLOCK)?;

        println!("Written 0xaa, read 0x{:02x}", buf[0]);
    }
}
