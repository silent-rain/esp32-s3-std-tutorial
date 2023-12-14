use std::{ffi::c_void, num::NonZeroU32};

use esp_idf_svc::{
    hal::{
        delay::{self},
        gpio::{InterruptType, PinDriver, Pull},
        peripherals::Peripherals,
        sys::EspError,
        task::notification::Notification,
        timer::{TimerConfig, TimerDriver},
    },
    sys::{gpio_install_isr_service, gpio_isr_handler_add, link_patches, ESP_INTR_FLAG_LEVEL1},
};

fn main() -> Result<(), EspError> {
    link_patches();

    // 设置日志级别
    log::set_max_level(log::LevelFilter::Info);

    let per = Peripherals::take()?;

    // 配置 GPIO 15 为输入模式，并启用内部上拉电阻
    let mut key = PinDriver::input(per.pins.gpio15)?;
    key.set_pull(Pull::Up)?;
    key.set_interrupt_type(InterruptType::AnyEdge)?;
    // key.enable_interrupt()?;

    // 配置定时器 0，使用单次模式，外部时钟源，和一个匿名的报警回调函数
    let timer_conf = TimerConfig::new().auto_reload(false).xtal(true);
    let mut timer = TimerDriver::new(per.timer00, &timer_conf)?;

    // Every half a second
    // timer.set_alarm(timer.tick_hz() / 2)?;
    // 设置定时器 0 的报警值为 1000
    timer.set_alarm(1000)?;

    // 启用定时器 0 的中断和报警
    timer.enable_interrupt()?;
    timer.enable_alarm(true)?;
    timer.enable(true)?;

    // 创建一个通知对象，用于从报警回调函数接收通知
    let notification = Notification::new();
    let notifier = notification.notifier();

    // 设置定时器 0 的报警回调函数，发送一个通知给主循环
    unsafe {
        timer.subscribe(move || {
            let bitset = 0b10001010101;
            notifier.notify_and_yield(NonZeroU32::new(bitset).unwrap());
        })?;
    }

    // 注意：这里会报错
    unsafe {
        // 安装 GPIO 中断服务例程，使用中断通道 1，中断优先级为 1
        gpio_install_isr_service(ESP_INTR_FLAG_LEVEL1 as i32);
        // 注册 GPIO 15 的中断处理函数到中断服务例程中，传递定时器 0 的句柄作为参数
        gpio_isr_handler_add(
            key.pin(),
            Some(gpio_isr_handler),
            &mut timer as *mut TimerDriver<'_> as *mut c_void,
        );
    }

    loop {
        // 等待通知
        let bitset = notification.wait(delay::BLOCK);

        if let Some(bitset) = bitset {
            println!("got event with bits {bitset:#b} from ISR");
            // 禁用中断
            // timer.disable_interrupt()?;
        }
    }
}

// 中断处理函数
unsafe extern "C" fn gpio_isr_handler(args: *mut std::ffi::c_void) {
    let timer = &mut *(args as *mut TimerDriver<'_>);
    timer.set_counter(1).unwrap();
}
