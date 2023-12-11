use std::sync::atomic::{AtomicBool, Ordering};

use esp_idf_svc::{
    hal::{
        delay::FreeRtos,
        gpio::{InterruptType, PinDriver, Pull},
        peripherals::Peripherals,
    },
    log::EspLogger,
    sys::{gpio_install_isr_service, gpio_isr_handler_add, link_patches, ESP_INTR_FLAG_LEVEL1},
};

static FLAG: AtomicBool = AtomicBool::new(false);

fn main() -> anyhow::Result<()> {
    link_patches();

    EspLogger::initialize_default();
    log::set_max_level(log::LevelFilter::Info);

    // Get the peripherals
    let peripherals = Peripherals::take()?;

    let mut light_sensor = PinDriver::input(peripherals.pins.gpio5)?;

    // 上拉电阻使能，以防止悬空状态
    light_sensor.set_pull(Pull::Up)?;
    // 表示在任何边沿（上升或下降）触发中断
    // light_sensor.set_interrupt_type(InterruptType::AnyEdge)?;
    // 以检测正边缘上的中断
    light_sensor.set_interrupt_type(InterruptType::PosEdge)?;

    unsafe {
        gpio_install_isr_service(ESP_INTR_FLAG_LEVEL1 as i32);

        gpio_isr_handler_add(
            light_sensor.pin(),
            Some(gpio_isr_handler),
            std::ptr::null_mut(),
        );
    }

    // 为 GPIO4 添加中断处理函数
    // gpio_isr_handler_add(GPIO_INPUT_PIN, gpio_isr_handler, (void*) GPIO_INPUT_PIN);

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
