use esp_idf_svc::{
    hal::{peripherals::Peripherals, sys::EspError},
    sys::{
        esp_deep_sleep_start, esp_sleep_enable_timer_wakeup, esp_timer_create,
        esp_timer_create_args_t, esp_timer_dispatch_t_ESP_TIMER_TASK, esp_timer_get_time,
        esp_timer_handle_t, esp_timer_start_periodic, link_patches, rtc_gpio_init,
        rtc_gpio_mode_t_RTC_GPIO_MODE_INPUT_ONLY, rtc_gpio_set_direction, rtc_gpio_set_level,
        soc_rtc_slow_clk_src_t_SOC_RTC_SLOW_CLK_SRC_XTAL32K,
    },
};

const RTC_GPIO_NUM: i32 = 4; // 选择一个RTC GPIO作为外部时钟输入
const EXT_CLK_FREQ: i32 = 32768; // 外部时钟的频率，单位Hz
const TIMER_INTERVAL_SEC: u64 = 5; // 定时器的间隔，单位秒

fn main() -> Result<(), EspError> {
    link_patches();

    // 设置日志级别
    log::set_max_level(log::LevelFilter::Info);

    let _per = Peripherals::take()?;

    unsafe {
        // 初始化RTC GPIO
        rtc_gpio_init(RTC_GPIO_NUM);
        rtc_gpio_set_direction(RTC_GPIO_NUM, rtc_gpio_mode_t_RTC_GPIO_MODE_INPUT_ONLY);
        rtc_gpio_set_level(RTC_GPIO_NUM, 0);

        // 选择外部时钟源
        rtc_clk_32k_enable_external(); // TODO: 这个函数不存在，rtc.h
        rtc_clk_32k_bootstrap(EXT_CLK_FREQ); // TODO: 这个函数不存在，rtc.h

        // 设置RTC定时器的时钟源和分频器
        rtc_clk_slow_freq_set(soc_rtc_slow_clk_src_t_SOC_RTC_SLOW_CLK_SRC_XTAL32K); // TODO: 这个函数不存在，rtc.h
        rtc_clk_divider_set(1); // TODO: 这个函数不存在，rtc.h

        // 创建一个周期性的定时器
        let timer_args = esp_timer_create_args_t {
            callback: Some(timer_callback),                       // 设置回调函数
            arg: std::ptr::null_mut(),                            // 设置参数为null
            dispatch_method: esp_timer_dispatch_t_ESP_TIMER_TASK, // 设置调度方法为任务
            name: "Measurement".as_ptr() as *const i8,            // 设置定时器的名称
            skip_unhandled_events: false,                         // 不跳过未处理的事件
        }; // 初始化定时器参数结构体
        let mut timer: esp_timer_handle_t = std::ptr::null_mut(); // 声明和初始化定时器句柄
        esp_timer_create(&timer_args as *const esp_timer_create_args_t, &mut timer); // 创建和初始化定时器
        esp_timer_start_periodic(timer, TIMER_INTERVAL_SEC * 1000000_u64); // 以指定的间隔启动定时器

        // 进入深度睡眠模式
        esp_sleep_enable_timer_wakeup(TIMER_INTERVAL_SEC * 1000000_u64);
        esp_deep_sleep_start();
    }
}

// 中断处理函数
unsafe extern "C" fn timer_callback(args: *mut std::ffi::c_void) {
    // 在这里写定时器的回调函数，例如打印时间或执行其他任务
    let time_since_boot = esp_timer_get_time();
    log::info!("Time since boot: {} us\n", time_since_boot);
}
