use esp_idf_svc::{
    hal::{
        delay::FreeRtos,
        ledc::{config::TimerConfig, LedcDriver, LedcTimerDriver},
        peripherals::Peripherals,
        prelude::FromValueType,
    },
    log::EspLogger,
    sys::link_patches,
};

fn main() -> anyhow::Result<()> {
    link_patches();
    EspLogger::initialize_default();
    log::set_max_level(log::LevelFilter::Info);

    let peripherals = Peripherals::take()?;

    // 配置 LEDC 的定时器
    let mut timer_driver = LedcTimerDriver::new(
        peripherals.ledc.timer0,
        // 设置 PWM 信号的频率为 25 kHz，分辨率为默认值（13 位）
        &TimerConfig::new().frequency(25.kHz().into()),
    )?;

    // 对于 LED 灯的控制，PWM 信号的频率会影响 LED 灯的亮度和闪烁感。
    // 如果频率太低，LED 灯会明显闪烁，影响视觉效果，甚至会引起眼睛疲劳。
    // 如果频率太高，LED 灯会看起来很暗，因为每个周期内的高电平时间很短。
    // 一般来说，LED 灯的适合的 PWM 频率范围是 50 Hz 到 1 kHz4。
    // 设置 PWM 信号的频率为 1 kHz
    timer_driver.set_frequency(1.kHz().into())?;

    println!("Configuring output channel");
    // 用于配置 LEDC 的通道
    let mut channel = LedcDriver::new(
        peripherals.ledc.channel0,
        timer_driver,
        // 将通道与 GPIO5 管脚关联，输出 PWM 信号
        peripherals.pins.gpio5,
    )?;

    log::info!("Starting duty-cycle loop");

    let max_duty = channel.get_max_duty();
    log::info!("Max Duty: {}", max_duty);

    // 开始循环改变 PWM 信号的占空比，从而改变 LED 灯的亮度;
    // 使用 for 循环来遍历一个数组的迭代器，该数组包含 0 到 5 的整数，使用 iter() 方法来创建一个迭代器，使用 cycle() 方法来使其无限循环，将每个元素赋值给 numerator 变量
    // for numerator in [0, 1, 2, 3, 4, 5].iter().cycle() {
    //     log::info!("Duty {numerator}/5");
    //     // 改变 PWM 信号的占空比
    //     channel.set_duty(max_duty * numerator / 5)?;

    //     // 延迟 2000 毫秒，实现 LED 灯的渐变效果
    //     FreeRtos::delay_ms(500);
    // }

    loop {
        for i in 0..=100 {
            // 改变 PWM 信号的占空比
            channel.set_duty(max_duty * i / 100)?;
            FreeRtos::delay_ms(10);
        }
        for i in 0..=100 {
            // 改变 PWM 信号的占空比
            channel.set_duty(max_duty * (100 - i) / 100)?;
            FreeRtos::delay_ms(10);
        }
        FreeRtos::delay_ms(10);
    }
}
