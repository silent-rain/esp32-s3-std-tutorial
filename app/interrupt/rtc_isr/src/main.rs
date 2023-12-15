use std::sync::atomic::{AtomicBool, Ordering};

use esp_idf_svc::{
    hal::{
        delay::FreeRtos,
        peripherals::Peripherals,
        sys::EspError,
        timer::{TimerConfig, TimerDriver},
    },
    sys::link_patches,
    sys::rtc_init,
};

static FLAG: AtomicBool = AtomicBool::new(false);

fn main() -> Result<(), EspError> {
    link_patches();

    // 设置日志级别
    log::set_max_level(log::LevelFilter::Info);

    let _per = Peripherals::take()?;

    // 初始化 RTC 模块
    unsafe {
        rtc_init();
    }

    // 设置 RTC 计数器的时钟源为 32.768 kHz 晶振，并设置分频比为 1
    unsafe {
        rtc_clk_set_xtal_freq(32);
        rtc_clk_set_divider(1);
    }

    // 设置 RTC 闹钟的值为当前 RTC 计数器的值加上 10 秒
    unsafe {
        let mut now: u64 = 0;
        rtc_time_get(&mut now);
        rtc_time_set_alarm(now + 10 * 32768);
    }

    // 设置 RTC 闹钟的中断模式为单次触发
    unsafe {
        rtc_time_set_alarm_mode(RtcAlarmMode::Single);
    }

    // 使能 RTC 闹钟中断
    unsafe {
        rtc_time_enable_alarm();
    }

    // 注册 RTC 闹钟中断的处理函数
    unsafe {
        esp_intr_alloc(
            EtsRtcCoreIntrSource as u32,
            EspIntrFlagLevel3 as u32,
            Some(rtc_alarm_handler),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
    }

    // 启动 RTC 计数器
    unsafe {
        rtc_time_start();
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

// 定义一个 RTC 闹钟中断的处理函数
fn rtc_alarm_handler() {
    // 在这里写入您想要执行的操作，例如打印一条信息
    println!("RTC alarm triggered!");
}
