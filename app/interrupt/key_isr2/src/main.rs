use std::sync::atomic::{AtomicBool, Ordering};

use esp_idf_svc::{
    hal::{
        delay::FreeRtos,
        gpio::{InterruptType, PinDriver, Pull},
        peripherals::Peripherals,
    },
    log::EspLogger,
    sys::{gpio_isr_handle_t, gpio_isr_register, link_patches, ESP_INTR_FLAG_LEVEL1},
};

static FLAG: AtomicBool = AtomicBool::new(false);

fn main() -> anyhow::Result<()> {
    link_patches();

    EspLogger::initialize_default();
    log::set_max_level(log::LevelFilter::Info);

    // Get the peripherals
    let peripherals = Peripherals::take()?;

    let mut key = PinDriver::input(peripherals.pins.gpio5)?;

    // 上拉电阻使能，以防止悬空状态
    key.set_pull(Pull::Up)?;
    // 表示在任何边沿（上升或下降）触发中断
    // key.set_interrupt_type(InterruptType::AnyEdge)?;
    // 以检测正边缘上的中断
    key.set_interrupt_type(InterruptType::PosEdge)?;

    // fn_: 这是一个 Option 类型的参数，它可以包含一个函数指针，或者是 None。这个函数指针指向一个接受一个 void* 类型的参数，没有返回值的函数。这个函数是您定义的中断处理函数，它会在 GPIO 引脚发生中断时被调用。如果传入 None，则表示取消之前注册的中断处理函数。
    // intr_alloc_flags: 这是一个 int 类型的值，用于指定分配中断的标志。您可以使用一个或多个（按位或）ESP_INTR_FLAG_* 值，来设置中断的优先级，共享模式，CPU核心等。您可以在 esp_intr_alloc.h 文件中查看更多信息。
    // arg: 这是一个 void* 类型的指针，用于传递给中断处理函数的参数。您可以根据您的需要，将任何类型的数据转换为 void* 类型，然后在中断处理函数中再转换回原来的类型。
    // handle: 这是一个 gpio_isr_handle_t 类型的指针，用于返回中断的句柄。如果不为 NULL，一个用于表示中断的句柄会被返回到这个指针指向的位置。您可以使用这个句柄，来调用 esp_intr_free 或 esp_intr_disable 函数，来释放或禁用中断。
    let handle: *mut gpio_isr_handle_t = std::ptr::null_mut();
    unsafe {
        // 注册中断处理函数
        // 注册GPIO中断处理程序，该处理程序是一个ISR。处理程序将附加到运行此函数的同一 CPU 核心。
        // 每当任何 GPIO中断发生时，都会调用此 ISR 函数。请参阅替代 gpio_install_isr_service() 和 gpio_isr_handler_add() API 以使驱动程序支持 per-GPIO ISR。
        gpio_isr_register(
            Some(gpio_isr_handler),
            std::ptr::null_mut(),
            ESP_INTR_FLAG_LEVEL1 as i32,
            handle,
        );
    }

    // Set up a variable that keeps track of press button count
    let mut count = 0_u32;

    log::info!("loop");
    loop {
        FreeRtos::delay_ms(10);
        // Check if global flag is asserted
        if FLAG.load(Ordering::Relaxed) {
            // Reset global flag
            FLAG.store(false, Ordering::Relaxed);
            // Update Press count and print
            count = count.wrapping_add(1);
            println!("Press Count {}", count);
        }
    }
}

// 中断处理函数
unsafe extern "C" fn gpio_isr_handler(_arg: *mut std::ffi::c_void) {
    FLAG.store(true, Ordering::Relaxed);
}
