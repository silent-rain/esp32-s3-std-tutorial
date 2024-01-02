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
    sys::{gpio_install_isr_service, gpio_isr_handler_add, link_patches, ESP_INTR_FLAG_LEVEL1},
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

    unsafe {
        gpio_install_isr_service(ESP_INTR_FLAG_LEVEL1 as i32);

        gpio_isr_handler_add(13, Some(gpio_isr_handler), std::ptr::null_mut());
    }

    loop {
        let s = serial::recv_string(&mut uart, BLOCK)?;
        log::info!("recv: {:?}", s);
    }
}

// 中断处理函数
unsafe extern "C" fn gpio_isr_handler(_arg: *mut std::ffi::c_void) {
    log::info!("isr");
}
