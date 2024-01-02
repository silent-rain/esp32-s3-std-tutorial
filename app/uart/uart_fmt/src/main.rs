use esp_idf_svc::{
    hal::{
        delay::FreeRtos,
        gpio,
        peripherals::Peripherals,
        uart::{
            config::{DataBits, StopBits},
            UartConfig, UartDriver,
        },
        units::Hertz,
    },
    io::Write,
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
        // 设置波特率
        .baudrate(Hertz(115_200))
        // 设置停止位为1个位
        .stop_bits(StopBits::STOP2)
        // 设置数据位数为8位
        .data_bits(DataBits::DataBits8)
        .parity_none();
    let mut uart = UartDriver::new(
        peripherals.uart1,
        tx,
        rx,
        Option::<gpio::Gpio0>::None,
        Option::<gpio::Gpio1>::None,
        &config,
    )?;

    uart.write(&[0xaa])?;

    std::fmt::Write::write_str(&mut uart, "xxxxxx")?;

    let number = 103;
    writeln!(uart, "Hello formatted string {}", number).unwrap();

    loop {
        FreeRtos::delay_ms(1000);
    }
}
