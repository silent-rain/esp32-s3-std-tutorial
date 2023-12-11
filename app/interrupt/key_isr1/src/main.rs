use std::sync::atomic::{AtomicBool, Ordering};

use esp_idf_svc::{
    hal::{
        delay::FreeRtos,
        gpio::{InterruptType, PinDriver, Pull},
        peripherals::Peripherals,
    },
    log::EspLogger,
    sys::link_patches,
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

    // 在收到中断通知后，需要在非中断上下文中，再次调用 PinDriver::enable_interrupt 方法，来重新启用中断，否则会触发看门狗定时器。
    unsafe { key.subscribe(gpio_int_callback)? }
    key.enable_interrupt()?;

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
fn gpio_int_callback() {
    FLAG.store(true, Ordering::Relaxed);
}
