#![allow(clippy::empty_loop)]

use esp_idf_svc::{
    hal::{delay::FreeRtos, peripherals::Peripherals},
    log::EspLogger,
    sys::link_patches,
};

const CONST: i32 = 0x66;
static STATIC: i32 = 0x66;

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    link_patches();

    // Bind the log crate to the ESP Logging facilities
    EspLogger::initialize_default();

    // 设置日志级别
    log::set_max_level(log::LevelFilter::Info);

    // Get the peripherals
    let _peripherals = Peripherals::take()?;

    let var = 0x66;
    let var_p = &var as *const i32 as usize as u32;
    let const_p = &CONST as *const i32 as usize as u32;
    let static_p = &STATIC as *const i32 as usize as u32;
    log::info!(
        "var_p={:?} const_p={:?} static_p={:?}",
        var_p,
        const_p,
        static_p,
    );

    log::info!("loop");
    loop {
        FreeRtos::delay_ms(1000);
    }
}
