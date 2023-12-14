use std::num::NonZeroU32;

use esp_idf_svc::{
    hal::{
        delay::{self},
        peripherals::Peripherals,
        sys::EspError,
        task::notification::Notification,
        timer::{TimerConfig, TimerDriver},
    },
    sys::link_patches,
};

fn main() -> Result<(), EspError> {
    link_patches();

    // 设置日志级别
    log::set_max_level(log::LevelFilter::Info);

    let per = Peripherals::take()?;

    // BaseClock for the Timer is the APB_CLK that is running on 80MHz at default
    // The default clock-divider is -> 80
    // default APB clk is available with the APB_CLK_FREQ constant
    let timer_conf = TimerConfig::new().auto_reload(true);
    let mut timer = TimerDriver::new(per.timer00, &timer_conf)?;

    // Every half a second
    timer.set_alarm(timer.tick_hz() / 2)?;

    // A safer abstraction over FreeRTOS/ESP-IDF task notifications.
    let notification = Notification::new();

    let notifier = notification.notifier();

    // Saftey: make sure the `Notification` object is not dropped while the subscription is active
    unsafe {
        timer.subscribe(move || {
            let bitset = 0b10001010101;
            notifier.notify_and_yield(NonZeroU32::new(bitset).unwrap());
        })?;
    }

    timer.enable_interrupt()?;
    timer.enable_alarm(true)?;
    timer.enable(true)?;

    loop {
        // Notify approach
        // The benefit with this approach over checking a global static variable is
        // that the scheduler can block the task, and quickly resume it when notified
        // so no spinlock is needed / the CPU does not waste cycles.
        let bitset = notification.wait(delay::BLOCK);

        if let Some(bitset) = bitset {
            println!("got event with bits {bitset:#b} from ISR");
            // 禁用中断
            timer.disable_interrupt()?;
        }
    }
}
