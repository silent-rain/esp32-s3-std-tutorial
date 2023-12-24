use esp_idf_svc::{
    hal::{
        delay::Ets,
        gpio::{OutputPin, PinDriver},
        peripheral::Peripheral,
        peripherals::Peripherals,
        rmt::{
            config::{CarrierConfig, DutyPercent, Loop, TransmitConfig},
            PinState, Pulse, PulseTicks, RmtChannel, TxRmtDriver, VariableLengthSignal,
        },
        units::FromValueType,
    },
    log::EspLogger,
    sys::link_patches,
};

fn main() -> anyhow::Result<()> {
    link_patches();
    EspLogger::initialize_default();
    log::set_max_level(log::LevelFilter::Info);

    println!("Starting APP!");

    let peripherals = Peripherals::take()?;
    let mut channel = peripherals.rmt.channel0;

    let mut buzzer = peripherals.pins.gpio17;
    let stop = peripherals.pins.gpio16;

    let carrier = CarrierConfig::new()
        // 设置载波的占空比为50%
        .duty_percent(DutyPercent::new(50)?)
        // 设置载波的频率为611赫兹
        .frequency(611.Hz());
    let mut config = TransmitConfig::new()
        .carrier(Some(carrier)) // 设置载波
        .looping(Loop::Endless) // 设置循环模式为无限循环
        .clock_divider(255); // 设置时钟分频器为255

    // 使用RMT模块和蜂鸣器发送摩尔斯电码信息
    let tx = send_morse_code(&mut channel, &mut buzzer, &config, "HELLO ")?;

    // 设置为输入模式
    let stop = PinDriver::input(stop)?;

    println!("Keep sending until pin {} is set low.", stop.pin());

    // 检查stop引脚的电平，如果是高电平，就执行循环体
    while stop.is_high() {
        Ets::delay_ms(100);
    }

    println!("Pin {} set to low. Stopped.", stop.pin());

    // 调用drop函数，释放tx变量，以便后续使用
    drop(tx);

    // Wait so the messages don't get garbled.
    Ets::delay_ms(3000);

    // 表示不循环发送
    println!("Saying GOODBYE!");
    config.looping = Loop::None;
    // 使用RMT模块和蜂鸣器发送摩尔斯电码信息
    send_morse_code(channel, buzzer, &config, "GOODBYE")?;

    Ok(())
}

/// 使用RMT模块和蜂鸣器发送摩尔斯电码信息
fn send_morse_code<'d>(
    channel: impl Peripheral<P = impl RmtChannel> + 'd,
    buzzer: impl Peripheral<P = impl OutputPin> + 'd,
    config: &TransmitConfig,
    message: &str,
) -> anyhow::Result<TxRmtDriver<'d>> {
    println!("Sending morse message '{message}'.");

    // 实例表示一个可变长度的信号，用于存储一系列的脉冲
    let mut signal = VariableLengthSignal::new();
    // 将字符串转换为脉冲的向量
    let pulses = str_pulses(message);
    // 将脉冲的向量转换为脉冲的引用的向量，因为VariableLengthSignal需要的是脉冲的引用
    let pulses: Vec<&Pulse> = pulses.iter().collect();
    signal.push(pulses)?;

    // 表示一个RMT发送驱动器，需要传入RMT通道，蜂鸣器和发送配置
    let mut tx = TxRmtDriver::new(channel, buzzer, config)?;
    // 开始发送信号
    tx.start(signal)?;

    // Return `tx` so we can release the pin and channel later.
    Ok(tx)
}

// 用于创建一个高电平的脉冲，持续时间为最大值
fn high() -> Pulse {
    Pulse::new(PinState::High, PulseTicks::max())
}

/// 用于创建一个低电平的脉冲，持续时间为最大值
fn low() -> Pulse {
    Pulse::new(PinState::Low, PulseTicks::max())
}

/// 定义一个枚举类型，表示摩尔斯电码的三种符号：点，划和单词间隔
enum Code {
    Dot,
    Dash,
    WordGap,
}

/// 为Code类型实现一个特征，用于将摩尔斯电码符号转换为脉冲并添加到脉冲的向量中
impl Code {
    /// 定义一个方法，接受一个脉冲的向量的可变引用作为参数
    pub fn push_pulse(&self, pulses: &mut Vec<Pulse>) {
        // 根据不同的摩尔斯电码符号，添加不同的脉冲序列到向量中
        match &self {
            // 点表示为一个高电平的脉冲和一个低电平的脉冲
            Code::Dot => pulses.extend_from_slice(&[high(), low()]),
            // 划表示为三个高电平的脉冲和一个低电平的脉冲
            Code::Dash => pulses.extend_from_slice(&[high(), high(), high(), low()]),
            // 单词间隔表示为六个低电平的脉冲
            Code::WordGap => pulses.extend_from_slice(&[low(), low(), low(), low(), low(), low()]),
        }
    }
}

/// 用于根据字符查找对应的摩尔斯电码符号的向量
fn find_codes(c: &char) -> &'static [Code] {
    // 遍历CODES常量中的元组，每个元组包含一个字符和一个摩尔斯电码符号的向量
    for (found, codes) in CODES.iter() {
        // 如果找到了匹配的字符，就返回对应的摩尔斯电码符号的向量
        if found == c {
            return codes;
        }
    }
    // 如果没有找到匹配的字符，就返回一个空的向量
    &[]
}

/// 定义一个函数，用于将字符串转换为脉冲的向量
fn str_pulses(s: &str) -> Vec<Pulse> {
    // 创建一个空的脉冲的向量
    let mut pulses = vec![];
    // 遍历字符串中的每个字符
    for c in s.chars() {
        for code in find_codes(&c) {
            // 将摩尔斯电码符号转换为脉冲并添加到脉冲的向量中
            code.push_pulse(&mut pulses);
        }

        // 在每个符号后面添加一个间隔，表示为两个低电平的脉冲
        pulses.push(low());
        pulses.push(low());
    }
    pulses
}

// 定义一个常量，表示一个元组的向量，每个元组包含一个字符和一个摩尔斯电码符号的向量
const CODES: &[(char, &[Code])] = &[
    (' ', &[Code::WordGap]),
    ('A', &[Code::Dot, Code::Dash]),
    ('B', &[Code::Dash, Code::Dot, Code::Dot, Code::Dot]),
    ('C', &[Code::Dash, Code::Dot, Code::Dash, Code::Dot]),
    ('D', &[Code::Dash, Code::Dot, Code::Dot]),
    ('E', &[Code::Dot]),
    ('F', &[Code::Dot, Code::Dot, Code::Dash, Code::Dot]),
    ('G', &[Code::Dash, Code::Dash, Code::Dot]),
    ('H', &[Code::Dot, Code::Dot, Code::Dot, Code::Dot]),
    ('I', &[Code::Dot, Code::Dot]),
    ('J', &[Code::Dot, Code::Dash, Code::Dash, Code::Dash]),
    ('K', &[Code::Dash, Code::Dot, Code::Dash]),
    ('L', &[Code::Dot, Code::Dash, Code::Dot, Code::Dot]),
    ('M', &[Code::Dash, Code::Dash]),
    ('N', &[Code::Dash, Code::Dot]),
    ('O', &[Code::Dash, Code::Dash, Code::Dash]),
    ('P', &[Code::Dot, Code::Dash, Code::Dash, Code::Dot]),
    ('Q', &[Code::Dash, Code::Dash, Code::Dot, Code::Dash]),
    ('R', &[Code::Dot, Code::Dash, Code::Dot]),
    ('S', &[Code::Dot, Code::Dot, Code::Dot]),
    ('T', &[Code::Dash]),
    ('U', &[Code::Dot, Code::Dot, Code::Dash]),
    ('V', &[Code::Dot, Code::Dot, Code::Dot, Code::Dash]),
    ('W', &[Code::Dot, Code::Dash, Code::Dash]),
    ('X', &[Code::Dash, Code::Dot, Code::Dot, Code::Dash]),
    ('Y', &[Code::Dash, Code::Dot, Code::Dash, Code::Dash]),
    ('Z', &[Code::Dash, Code::Dash, Code::Dot, Code::Dot]),
];
