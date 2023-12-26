use esp_idf_svc::{
    hal::{
        delay::FreeRtos,
        gpio::{Gpio4, Input, PinDriver, Pull},
        ledc::{config::TimerConfig, LedcDriver, LedcTimerDriver, Resolution},
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

    let mut button = PinDriver::input(peripherals.pins.gpio4)?;
    // 上拉电阻使能，以防止悬空状态
    button.set_pull(Pull::Up)?;

    // 配置 LEDC 的定时器
    let timer_driver = LedcTimerDriver::new(
        peripherals.ledc.timer0,
        // 设置 PWM 信号的频率为 50 Hz
        &TimerConfig::new()
            .frequency(50.Hz())
            // 使用10位分辨率意味着有1024个可能的占空比值
            .resolution(Resolution::Bits10),
    )?;

    println!("Configuring output channel");
    // 用于配置 LEDC 的通道
    let mut channel = LedcDriver::new(
        peripherals.ledc.channel0,
        timer_driver,
        // 将通道与 GPIO5 管脚关联，输出 PWM 信号
        peripherals.pins.gpio5,
    )?;

    // 总周期为20ms 频率为 50 Hz max_duty=1023
    let max_duty = channel.get_max_duty();
    log::info!("Max Duty: {}", max_duty);

    let mut angle = -90;

    // 20ms         max_duty=1023
    // 0.5ms  -90°    (0.5ms / 20ms) * 1024 ≈ 25.6
    // 1.5ms  0°     (1.5ms / 20ms) * 1024 ≈ 76.8
    // 2ms    45°    (2ms / 20ms) * 1024 ≈ 102.4
    // 2.5ms  90°    (2.5ms / 20ms) * 1024 ≈ 128

    // 舵机重置
    log::info!("angle: {}  duty: {}", 0, 25.6);
    channel.set_duty(26)?;
    loop {
        if !get_key_down(&mut button) {
            FreeRtos::delay_ms(10);
            continue;
        }
        angle += 30;
        if angle > 90 {
            angle = -90
        }
        let duty = if angle < 0 {
            ((-1.0 * angle as f32 - 90.0) * (76.8 - 25.6) / (0.0 - (-90.0)) + 25.6) as u32
        } else {
            (angle as f32 * (128.0 - 76.8) / (90.0 - 0.0) + 76.8) as u32
        };

        log::info!("angle: {}  duty: {}", angle, duty);
        channel.set_duty(duty)?;
    }
}

/// 按键按下状态
fn get_key_down(key: &mut PinDriver<'_, Gpio4, Input>) -> bool {
    if key.is_low() {
        // 按键按下抖动
        FreeRtos::delay_ms(20);
        // 按着不动, 松手后跳出循环
        while key.is_low() {}
        // 按键松开抖动
        FreeRtos::delay_ms(20);
        return true;
    }
    false
}
