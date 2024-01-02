use std::ffi::c_int;

use esp_idf_svc::{
    hal::{
        delay::BLOCK,
        gpio,
        interrupt::InterruptType,
        peripherals::Peripherals,
        uart::{
            config::{DataBits, StopBits},
            UartConfig, UartDriver,
        },
        units::Hertz,
    },
    log::EspLogger,
    sys::{
        link_patches, uart_driver_install, uart_enable_rx_intr, uart_isr_handle_t, uart_port_t,
        QueueHandle_t, UART_NUM_1,
    },
};

const UART_NUM: uart_port_t = UART_NUM_1 as i32; // 使用 UART0 端口
const BUF_SIZE: c_int = 1024; // 设置接收缓冲区的大小为 1024 字节
const UART_QUEUE_SIZE: c_int = 10; // 设置接收队列的大小为 10

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

    let mut config = UartConfig::new()
        .baudrate(Hertz(115_200))
        .stop_bits(StopBits::STOP2)
        .data_bits(DataBits::DataBits8)
        .parity_none();
    config.intr_flags = InterruptType::Iram.into();

    let mut uart = UartDriver::new(
        peripherals.uart1,
        tx,
        rx,
        Option::<gpio::Gpio0>::None,
        Option::<gpio::Gpio1>::None,
        &config,
    )?;

    unsafe {
        // 用来保存接收队列的句柄
        let uart_queue: *mut QueueHandle_t = todo!();

        uart_driver_install(UART_NUM, BUF_SIZE, BUF_SIZE, UART_QUEUE_SIZE, uart_queue, 0);

        // 设置uart中断回调函数
        // uart_isr_register(UART_NUM, uart_isr, NULL, ESP_INTR_FLAG_IRAM, NULL);
        //

        // 使能uart接收中断
        uart_enable_rx_intr(UART_NUM);
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
