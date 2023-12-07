#![allow(clippy::empty_loop)]

use esp_idf_svc::{
    hal::{
        peripherals::Peripherals,
        task::block_on,
        timer::{TimerConfig, TimerDriver},
    },
    log::EspLogger,
    sys::link_patches,
};

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

    let mut timer = TimerDriver::new(peripherals.timer00, &TimerConfig::new())?;

    block_on(async {
        loop {
            // Every second
            timer.delay(timer.tick_hz()).await?;

            // 延迟10ms
            // timer.delay(timer.tick_hz() / 100).await?;
            log::info!("Tick");
        }
    })
}
