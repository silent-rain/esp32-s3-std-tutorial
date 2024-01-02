use std::fmt::Write;

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

    let config = UartConfig::new()
        .baudrate(Hertz(115_200))
        .stop_bits(StopBits::STOP2)
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
    uart.write_char('A')?;
    uart.write_fmt(format_args!("{} foo {:?}", 1, 2))?;
    uart.write_fmt(format_args!("{}", 123))?;
    uart.write_char('\n')?;
    uart.write_str("aaa\nbbb\n")?;

    // 自定义
    serial::send_byte(&mut uart, b'X')?;
    serial::send_byte(&mut uart, b'\n')?;
    serial::send_bytes(&mut uart, &[b'X', b'Y', b'Z', b'\n'])?;
    serial::send_bytes(&mut uart, "xyz\n".as_bytes())?;
    serial::send_string(&mut uart, "test\n")?;
    serial::send_number(&mut uart, 34567)?;

    loop {
        let mut buf = [0_u8; 1];
        uart.read(&mut buf, BLOCK)?;
        log::info!("recv: {:?}", buf);

        let s = serial::recv_string(&mut uart, BLOCK)?;
        log::info!("recv s: {:?}", s);
    }
}
