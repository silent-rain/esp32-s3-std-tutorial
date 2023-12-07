//! A simple example to change colours of a WS2812/NeoPixel compatible LED.
//!
//! This example demonstrates the use of [`FixedLengthSignal`][crate::rmt::FixedLengthSignal] which
//! lives on the stack and requires a known length before creating it.
//!
//! There is a similar implementation in the esp-idf project:
//! https://github.com/espressif/esp-idf/tree/20847eeb96/examples/peripherals/rmt/led_strip
//!
//! Datasheet (PDF) for a WS2812, which explains how the pulses are to be sent:
//! https://cdn-shop.adafruit.com/datasheets/WS2812.pdf

use core::time::Duration;

use anyhow::{bail, Result};
use esp_idf_hal::rmt::RmtChannel;
use esp_idf_hal::{
    gpio::OutputPin,
    peripheral::Peripheral,
    rmt::{config::TransmitConfig, FixedLengthSignal, PinState, Pulse, TxRmtDriver},
    sys::EspError,
};

// fn main() {
//     esp_idf_hal::sys::link_patches();

//     let peripherals = Peripherals::take()?;

//     // 3 seconds white at 10% brightness
//     neopixel(Rgb::new(25, 25, 25), &mut tx)?;
//

//     // infinite rainbow loop at 20% brightness
//     (0..360).cycle().try_for_each(|hue| {
//         FreeRtos::delay_ms(10);
//         let rgb = Rgb::from_hsv(hue, 100, 20)?;
//         neopixel(rgb, &mut tx)
//     })
// }
pub struct NeoPixel<'d> {
    tx: TxRmtDriver<'d>,
    data: Vec<Rgb>, // 灯珠颜色集合
}

impl<'d> NeoPixel<'d> {
    /// 创建对象
    pub fn new<PIN, OP, C, RC>(
        pin: PIN,   // RGB LED 灯的引脚
        channel: C, // RMT 通道
        num: usize, // RGB LED 灯的数量
    ) -> Result<Self>
    where
        PIN: Peripheral<P = OP> + 'd,
        OP: OutputPin,
        C: Peripheral<P = RC> + 'd,
        RC: RmtChannel,
    {
        // Onboard RGB LED pin
        // ESP32-C3-DevKitC-02 gpio8
        // ESP32-C3-DevKit-RUST-1 gpio2
        // ESP32-S3-DevKitC-1 gpio48
        // let led = peripherals.pins.gpio2;
        // let channel = peripherals.rmt.channel0;
        let config = TransmitConfig::new().clock_divider(1);
        let tx = TxRmtDriver::new(channel, pin, &config)?;

        let data = vec![Rgb::new(0, 0, 0); num];
        Ok(NeoPixel { tx, data })
    }

    /// 设置指定灯珠颜色
    pub fn set_color(&mut self, index: usize, color: Rgb) -> Result<()> {
        if index >= self.data.len() {
            bail!("索引超出范围");
        }
        self.data[index] = color;
        Ok(())
    }

    /// 设置所有灯珠颜色
    pub fn set_all_color(&mut self, color: Rgb) {
        for item in self.data.iter_mut() {
            *item = color.clone()
        }
    }

    // 用于设置某个 RGB LED 灯的颜色，并立即显示
    pub fn set_color_and_show(&mut self, index: usize, color: Rgb) -> Result<()> {
        // 设置颜色数据
        self.set_color(index, color)?;
        // 显示颜色数据
        self.show()?;
        // 返回成功
        Ok(())
    }

    /// 显示所有 RGB LED 灯的颜色
    pub fn show(&mut self) -> Result<()> {
        // 遍历 data 数组中的每个颜色数据
        let data = self.data.clone();
        for color in data.into_iter() {
            // 调用 send 方法，发送数据给对应的灯珠
            self.send(color)
                .expect("Error: Unable to send data to ws2812b");
        }

        // 添加一个复位信号，用于结束数据的发送
        // let ticks_hz = self.tx.counter_clock()?;
        // let (high_pulse, low_pulse) = (
        //     Pulse::new_with_duration(ticks_hz, PinState::High, &Duration::from_nanos(10))?,
        //     Pulse::new_with_duration(ticks_hz, PinState::Low, &Duration::from_nanos(20))?,
        // );
        // let mut signal = FixedLengthSignal::<1>::new();
        // signal.set(0, &(high_pulse, low_pulse))?;
        // self.tx
        //     .start_blocking(&signal)
        //     .expect("Error: Unable to write to rmt");
        Ok(())
    }

    // 用于清除所有 RGB LED 灯的颜色
    pub fn clear(&mut self) -> Result<()> {
        // 将所有颜色数据设置为黑色（全暗）
        for i in 0..self.data.len() {
            self.data[i] = Rgb { r: 0, g: 0, b: 0 };
        }
        self.show()?;

        Ok(())
    }

    /// 发送指令
    fn send(&mut self, rgb: Rgb) -> Result<(), EspError> {
        let color: u32 = rgb.into();
        let ticks_hz = self.tx.counter_clock()?;

        // 定义 WS2812B 的时序参数，单位为 ns
        let (t0h, t0l, t1h, t1l) = (
            Pulse::new_with_duration(ticks_hz, PinState::High, &Duration::from_nanos(350))?,
            Pulse::new_with_duration(ticks_hz, PinState::Low, &Duration::from_nanos(800))?,
            Pulse::new_with_duration(ticks_hz, PinState::High, &Duration::from_nanos(700))?,
            Pulse::new_with_duration(ticks_hz, PinState::Low, &Duration::from_nanos(600))?,
        );

        // 创建一个固定长度的信号，用于发送一个颜色数据
        let mut signal = FixedLengthSignal::<24>::new();
        for i in (0..24).rev() {
            let p = 2_u32.pow(i);
            let bit: bool = p & color != 0;
            let (high_pulse, low_pulse) = if bit { (t1h, t1l) } else { (t0h, t0l) };
            signal.set(23 - i as usize, &(high_pulse, low_pulse))?;
        }
        // 通过 RMT 驱动发送脉冲信号
        self.tx.start_blocking(&signal)?;
        Ok(())
    }
}

/// RGB 结构体
#[derive(Clone)]
pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Rgb {
    /// 创建 RGB 对象
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Converts hue, saturation, value to RGB
    pub fn from_hsv(h: u32, s: u32, v: u32) -> Result<Self> {
        if h > 360 || s > 100 || v > 100 {
            bail!("The given HSV values are not in valid range");
        }
        let s = s as f64 / 100.0;
        let v = v as f64 / 100.0;
        let c = s * v;
        let x = c * (1.0 - (((h as f64 / 60.0) % 2.0) - 1.0).abs());
        let m = v - c;
        let (r, g, b) = match h {
            0..=59 => (c, x, 0.0),
            60..=119 => (x, c, 0.0),
            120..=179 => (0.0, c, x),
            180..=239 => (0.0, x, c),
            240..=299 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };
        Ok(Self {
            r: ((r + m) * 255.0) as u8,
            g: ((g + m) * 255.0) as u8,
            b: ((b + m) * 255.0) as u8,
        })
    }
}

impl From<Rgb> for u32 {
    /// Convert RGB to u32 color value
    ///
    /// e.g. rgb: (1,2,4)
    /// G        R        B
    /// 7      0 7      0 7      0
    /// 00000010 00000001 00000100
    fn from(rgb: Rgb) -> Self {
        ((rgb.r as u32) << 16) | ((rgb.g as u32) << 8) | rgb.b as u32
    }
}
