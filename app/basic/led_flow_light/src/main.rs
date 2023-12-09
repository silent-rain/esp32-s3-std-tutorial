#![allow(clippy::empty_loop)]

use esp_idf_svc::{
    hal::{
        delay::FreeRtos,
        gpio::{Gpio4, Gpio5, Gpio6, Gpio7, Output, PinDriver},
        peripherals::Peripherals,
    },
    log::EspLogger,
    sys::link_patches,
};

trait ErasePinDriver {
    fn set_high(&mut self) -> anyhow::Result<()>;
    fn set_low(&mut self) -> anyhow::Result<()>;
}

struct EraseGpio4<'d>(PinDriver<'d, Gpio4, Output>);

impl<'d> ErasePinDriver for EraseGpio4<'d> {
    fn set_high(&mut self) -> anyhow::Result<()> {
        Ok(self.0.set_high()?)
    }
    fn set_low(&mut self) -> anyhow::Result<()> {
        Ok(self.0.set_low()?)
    }
}

struct EraseGpio5<'d>(PinDriver<'d, Gpio5, Output>);

impl<'d> ErasePinDriver for EraseGpio5<'d> {
    fn set_high(&mut self) -> anyhow::Result<()> {
        Ok(self.0.set_high()?)
    }
    fn set_low(&mut self) -> anyhow::Result<()> {
        Ok(self.0.set_low()?)
    }
}

struct EraseGpio6<'d>(PinDriver<'d, Gpio6, Output>);

impl<'d> ErasePinDriver for EraseGpio6<'d> {
    fn set_high(&mut self) -> anyhow::Result<()> {
        Ok(self.0.set_high()?)
    }
    fn set_low(&mut self) -> anyhow::Result<()> {
        Ok(self.0.set_low()?)
    }
}

struct EraseGpio7<'d>(PinDriver<'d, Gpio7, Output>);

impl<'d> ErasePinDriver for EraseGpio7<'d> {
    fn set_high(&mut self) -> anyhow::Result<()> {
        Ok(self.0.set_high()?)
    }
    fn set_low(&mut self) -> anyhow::Result<()> {
        Ok(self.0.set_low()?)
    }
}

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    link_patches();

    // Bind the log crate to the ESP Logging facilities
    EspLogger::initialize_default();

    // 设置日志级别
    log::set_max_level(log::LevelFilter::Info);

    // Get the peripherals
    let peripherals = Peripherals::take()?;

    // 创建要闪烁的LED阵列
    let mut leds: [Box<dyn ErasePinDriver>; 4] = [
        Box::new(EraseGpio4(PinDriver::output(peripherals.pins.gpio4)?)),
        Box::new(EraseGpio5(PinDriver::output(peripherals.pins.gpio5)?)),
        Box::new(EraseGpio6(PinDriver::output(peripherals.pins.gpio6)?)),
        Box::new(EraseGpio7(PinDriver::output(peripherals.pins.gpio7)?)),
    ];

    log::warn!("loop");
    loop {
        for led in leds.iter_mut() {
            led.set_high()?;
            FreeRtos::delay_ms(500);
        }
        for led in leds.iter_mut() {
            led.set_low()?;
            FreeRtos::delay_ms(200);
        }
        FreeRtos::delay_ms(1000);
    }
}
