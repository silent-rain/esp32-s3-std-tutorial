//! # SDK配置禁用开门狗
//!
//! ## 修改配置
//! ```text
//! vim sdkconfig
//!
//! # 禁用看门狗定时器
//! # CONFIG_INT_WDT=n
//! # CONFIG_ESP_TASK_WDT=n
//! ```
//!
//! ## 运行
//! ```shell
//! cargo run --bin sdkconfig_disable_wdg
//! ```
#![allow(clippy::empty_loop)]
use std::time::Duration;

use esp_idf_svc::{
    hal::{
        peripherals::Peripherals,
        task::{self, watchdog::TWDTConfig},
    },
    log::EspLogger,
    sys::link_patches,
};

fn main() {
    // It is necessary to call this function once.
    // Otherwise some patches to the runtime implemented by esp-idf-sys might not link properly.
    // See https://github.com/esp-rs/esp-idf-template/issues/71
    link_patches();

    // Bind the log crate to the ESP Logging facilities
    EspLogger::initialize_default();

    // 设置日志级别为 INFO
    log::set_max_level(log::LevelFilter::Info);

    // Get the peripherals
    let peripherals = Peripherals::take().unwrap();

    // 禁用开门狗
    let config = TWDTConfig {
        duration: Duration::from_secs(2),
        panic_on_trigger: true,
        subscribed_idle_tasks: enumset::EnumSet::empty(),
    };
    let driver = task::watchdog::TWDTDriver::new(peripherals.twdt, &config).unwrap();
    // 停止 TWDT 并释放资源
    drop(driver);

    log::info!("Hello, world!");
    loop {}
}
