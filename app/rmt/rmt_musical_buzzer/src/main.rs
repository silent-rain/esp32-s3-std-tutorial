use std::time::Duration;

use esp_idf_svc::{
    hal::{
        delay::{Ets, FreeRtos},
        peripherals::Peripherals,
        rmt::{
            config::{Loop, TransmitConfig},
            FixedLengthSignal, PinState, Pulse, PulseTicks, TxRmtDriver,
        },
    },
    log::EspLogger,
    sys::link_patches,
};
use notes::*;

fn main() -> anyhow::Result<()> {
    link_patches();
    EspLogger::initialize_default();
    log::set_max_level(log::LevelFilter::Info);

    println!("Starting APP!");

    let peripherals = Peripherals::take()?;
    let buzzer = peripherals.pins.gpio17;
    // 获取RMT通道0
    let channel = peripherals.rmt.channel0;
    // 创建一个无限循环发送的配置
    let config = TransmitConfig::new().looping(Loop::Endless);
    // 创建一个RMT发送驱动，用于控制蜂鸣器
    let mut tx: TxRmtDriver<'static> = TxRmtDriver::new(channel, buzzer, &config)?;

    loop {
        // 播放《欢乐颂》
        play_song(&mut tx, ODE_TO_JOY)?;
        Ets::delay_ms(3000);
    }
}

/// 播放《欢乐颂》
pub fn play_song(tx: &mut TxRmtDriver<'static>, song: &[NoteValue]) -> anyhow::Result<()> {
    for note_value in song {
        // 播放一个音符，传入频率和持续时间
        play_note(tx, note_value.note.0, note_value.duration)?;
    }
    Ok(())
}

/// 播放一个音符，传入频率和持续时间
pub fn play_note(
    tx: &mut TxRmtDriver<'static>,
    pitch: u16,
    duration: Duration,
) -> anyhow::Result<()> {
    // 计算蜂鸣器的频率。
    // 获取时钟频率
    let ticks_hz = tx.counter_clock()?;
    // 计算每个脉冲的时长
    let tick_count = (ticks_hz.0 as u128 / pitch as u128 / 2_u128) as u16;
    // 创建一个脉冲时长的实例
    let ticks = PulseTicks::new(tick_count)?;

    // 在滴答声持续时间内添加高脉冲和低脉冲。
    // 创建一个高电平的脉冲
    let on = Pulse::new(PinState::High, ticks);
    // 创建一个低电平的脉冲
    let off = Pulse::new(PinState::Low, ticks);
    // 创建一个固定长度的信号
    let mut signal = FixedLengthSignal::<1>::new();
    // 设置信号的第0位为高低电平的脉冲
    signal.set(0, &(on, off))?;

    // 在80%的持续时间内播放该音符。
    // 开始发送信号
    tx.start(signal)?;
    // 延迟80%的持续时间
    // Ets::delay_ms((80 * duration.as_millis() / 100) as u32);
    FreeRtos::delay_ms((80 * duration.as_millis() / 100) as u32);

    // 音符之间的小停顿，为指定持续时间的20%。
    // 停止发送信号
    tx.stop()?;
    // 延迟20%的持续时间
    // Ets::delay_ms((20 * duration.as_millis() / 100) as u32);
    FreeRtos::delay_ms((20 * duration.as_millis() / 100) as u32);

    Ok(())
}

/// 表示一个音符的频率，以赫兹为单位。
#[derive(Debug)]
pub struct Note(u16);

/// 包含了一些常用的音符常量，如A4，B4，C5等
pub mod notes {
    use crate::Note;

    #[allow(dead_code)]
    pub const A4: Note = Note(440);
    pub const AS4: Note = Note(466);
    pub const B4: Note = Note(494);
    pub const C5: Note = Note(523);
    pub const CS5: Note = Note(554);
    pub const D5: Note = Note(587);
    pub const DS5: Note = Note(622);
    pub const E5: Note = Note(659);
    pub const F5: Note = Note(698);
    pub const FS5: Note = Note(740);
    pub const G5: Note = Note(784);
    pub const GS5: Note = Note(831);
    pub const A5: Note = Note(880);
}

/// 表示一个音符的频率和持续时间，以毫秒为单位。
#[derive(Debug)]
pub struct NoteValue {
    note: Note,
    duration: Duration,
}

// 方便地创建NoteValue实例，它接受两个参数，分别是音符常量和持续时间。
macro_rules! n {
    ($n: expr, $duration: expr) => {
        NoteValue {
            note: $n,
            duration: Duration::from_millis($duration),
        }
    };
}

/// 它包含了《欢乐颂》的音符序列，它是NoteValue类型的切片。
/// 它使用了 n! 宏来创建每个音符的实例，传入音符常量和持续时间。
const ODE_TO_JOY: &[NoteValue] = &[
    n!(FS5, 400),
    n!(FS5, 600),
    n!(G5, 400),
    n!(A5, 400),
    n!(A5, 400),
    n!(G5, 400),
    n!(FS5, 400),
    n!(E5, 400),
    n!(D5, 400),
    n!(D5, 400),
    n!(E5, 400),
    n!(FS5, 400),
    n!(FS5, 400),
    n!(FS5, 200),
    n!(E5, 200),
    n!(E5, 800),
    n!(FS5, 400),
    n!(FS5, 600),
    n!(G5, 400),
    n!(A5, 400),
    n!(A5, 400),
    n!(G5, 400),
    n!(FS5, 400),
    n!(E5, 400),
    n!(D5, 400),
    n!(D5, 400),
    n!(E5, 400),
    n!(FS5, 400),
    n!(E5, 400),
    n!(E5, 200),
    n!(D5, 200),
    n!(D5, 800),
    n!(E5, 400),
    n!(E5, 400),
    n!(FS5, 400),
    n!(D5, 400),
    n!(E5, 400),
    n!(FS5, 200),
    n!(G5, 200),
    n!(FS5, 400),
    n!(D5, 400),
    n!(E5, 400),
    n!(FS5, 200),
    n!(G5, 200),
    n!(FS5, 400),
    n!(E5, 400),
    n!(D5, 400),
    n!(E5, 400),
    n!(A4, 400),
    n!(A4, 400),
    n!(FS5, 400),
    n!(FS5, 600),
    n!(G5, 400),
    n!(A5, 400),
    n!(A5, 400),
    n!(G5, 400),
    n!(FS5, 400),
    n!(E5, 400),
    n!(D5, 400),
    n!(D5, 400),
    n!(E5, 400),
    n!(FS5, 400),
    n!(E5, 400),
    n!(E5, 200),
    n!(D5, 200),
    n!(D5, 800),
];
