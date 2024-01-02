use esp_idf_svc::{
    hal::{
        gpio,
        peripherals::Peripherals,
        task::block_on,
        uart::{AsyncUartDriver, UartConfig},
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
    let config = UartConfig::new().baudrate(Hertz(115_200));
    let uart = AsyncUartDriver::new(
        peripherals.uart1,
        tx,
        rx,
        Option::<gpio::Gpio0>::None,
        Option::<gpio::Gpio1>::None,
        &config,
    )?;

    block_on(async {
        loop {
            uart.write(&[0xaa]).await?;

            let mut buf = [0_u8; 1];
            uart.read(&mut buf).await?;

            println!("Written 0xaa, read 0x{:02x}", buf[0]);
        }
    })
}
