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

    // 总周期为20ms max_duty=1023
    let max_duty = channel.get_max_duty();
    log::info!("Max Duty: {}", max_duty);

    let mut angle = 0;
    loop {
        if !get_key_down(&mut button) {
            FreeRtos::delay_ms(10);
            continue;
        }
        angle += 30;
        if angle > 180 {
            angle = 0
        }

        let duty = angle * max_duty / 180;
        log::info!("angle: {}  duty: {}", angle, duty);
        channel.set_duty(duty)?;
        FreeRtos::delay_ms(1000);
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
