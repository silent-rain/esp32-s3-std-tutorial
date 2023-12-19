use std::{sync::Arc, time::Duration};

use esp_idf_svc::{
    hal::{
        ledc::{config::TimerConfig, LedcDriver, LedcTimerDriver},
        peripherals::Peripherals,
        prelude::FromValueType,
    },
    log::EspLogger,
    sys::link_patches,
};

const CYCLES: usize = 3;

fn main() -> anyhow::Result<()> {
    link_patches();
    EspLogger::initialize_default();
    log::set_max_level(log::LevelFilter::Info);

    let peripherals = Peripherals::take()?;

    println!("Setting up PWM output channels");

    let config = TimerConfig::new().frequency(25.kHz().into());

    let timer = Arc::new(LedcTimerDriver::new(peripherals.ledc.timer0, &config)?);

    let channel0 = LedcDriver::new(
        peripherals.ledc.channel0,
        timer.clone(),
        peripherals.pins.gpio4,
    )?;

    let channel1 = LedcDriver::new(peripherals.ledc.channel1, timer, peripherals.pins.gpio5)?;

    println!("Spawning PWM threads");

    let thread0 = std::thread::Builder::new()
        .stack_size(7000)
        .spawn(move || cycle_duty(channel0, CYCLES, "PWM 0", Duration::from_millis(1000)))?;
    let thread1 = std::thread::Builder::new()
        .stack_size(7000)
        .spawn(move || cycle_duty(channel1, CYCLES, "PWM 1", Duration::from_millis(1750)))?;

    println!("Waiting for PWM threads");

    thread0.join().unwrap()?;
    thread1.join().unwrap()?;

    println!("Joined PWM threads");

    println!("Done");

    loop {
        // Don't let the idle task starve and trigger warnings from the watchdog.
        std::thread::sleep(Duration::from_millis(100));
    }
}

fn cycle_duty(
    mut pwm: LedcDriver<'_>,
    times: usize,
    log_prefix: &str,
    sleep: Duration,
) -> anyhow::Result<()> {
    let max_duty = pwm.get_max_duty();

    for cycle in 0..times {
        println!("{log_prefix} cycle: {cycle}");

        for numerator in [0, 1, 2, 3, 4, 5].iter() {
            println!("{log_prefix} duty: {numerator}/5");
            pwm.set_duty(max_duty * numerator / 5)?;
            std::thread::sleep(sleep);
        }
    }

    Ok(())
}
