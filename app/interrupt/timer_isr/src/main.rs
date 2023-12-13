use std::sync::atomic::{AtomicBool, Ordering};

use esp_idf_svc::{
    hal::{
        delay::FreeRtos,
        peripherals::Peripherals,
        sys::EspError,
        timer::{TimerConfig, TimerDriver},
    },
    sys::{
        gptimer_enable, gptimer_event_callbacks_t, gptimer_register_event_callbacks, gptimer_t,
        link_patches,
    },
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

    // timer_group: CONFLICT! driver_ng is not allowed to be used with the legacy driver
    unsafe {
        gptimer_enable(&mut timer as *mut TimerDriver<'_> as *mut gptimer_t);

        gptimer_register_event_callbacks(
            &mut timer as *mut TimerDriver<'_> as *mut gptimer_t, // 转换为原始指针
            gptimer_event_callback as *const gptimer_event_callbacks_t,
            std::ptr::null_mut(),
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

fn gptimer_event_callback() {
    FLAG.store(true, Ordering::Relaxed);
}
