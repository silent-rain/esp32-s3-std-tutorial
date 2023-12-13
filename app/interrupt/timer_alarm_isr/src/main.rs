use std::sync::atomic::{AtomicBool, Ordering};

use esp_idf_svc::{
    hal::{
        delay::FreeRtos,
        peripherals::Peripherals,
        sys::EspError,
        timer::{TimerConfig, TimerDriver},
    },
    sys::link_patches,
};

static FLAG: AtomicBool = AtomicBool::new(false);

fn main() -> Result<(), EspError> {
    link_patches();

    // 设置日志级别
    log::set_max_level(log::LevelFilter::Info);

    let per = Peripherals::take()?;

    // BaseClock for the Timer is the APB_CLK that is running on 80MHz at default
    // The default clock-divider is -> 80
    // default APB clk is available with the APB_CLK_FREQ constant
    // let timer_conf = config::Config::new().auto_reload(true);
    let timer_conf = TimerConfig::new().auto_reload(true);
    let mut timer = TimerDriver::new(per.timer00, &timer_conf)?;

    // Every half a second
    timer.set_alarm(timer.tick_hz() / 2)?;

    // Saftey: make sure the `Notification` object is not dropped while the subscription is active
    unsafe {
        timer.subscribe(move || {
            FLAG.store(true, Ordering::Relaxed);
        })?;
    }

    timer.enable_interrupt()?;
    timer.enable_alarm(true)?;
    timer.enable(true)?;

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
