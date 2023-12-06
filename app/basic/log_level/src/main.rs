#![allow(clippy::empty_loop)]

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // 设置日志级别
    log::set_max_level(log::LevelFilter::Warn);

    log::trace!("this is trace!");
    log::debug!("this is debug!");
    log::info!("this is info!");
    log::warn!("this is warn!");
    log::error!("this is error!");
    loop {}
}
