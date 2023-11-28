//! # 查看 esp32-s3 内存以及外存大小
//! ```rust
//! cargo run --bin hardware_memory_size
//! ```
use esp_idf_svc::{
    hal::peripherals::Peripherals,
    log::EspLogger,
    sys::{
        esp_image_flash_size_t_ESP_IMAGE_FLASH_SIZE_16MB, heap_caps_get_free_size,
        heap_caps_get_total_size, link_patches, MALLOC_CAP_8BIT, MALLOC_CAP_SPIRAM,
    },
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
    let _peripherals = Peripherals::take().unwrap();

    // 获取内存的总大小和可用大小
    let total_memory = unsafe { heap_caps_get_total_size(MALLOC_CAP_8BIT) };
    let free_memory = unsafe { heap_caps_get_free_size(MALLOC_CAP_8BIT) };

    // 获取外存的总大小和可用大小
    let total_psram = unsafe { heap_caps_get_total_size(MALLOC_CAP_SPIRAM) };
    let free_psram = unsafe { heap_caps_get_free_size(MALLOC_CAP_SPIRAM) };

    // 获取图像的flash大小
    let _image_flash = unsafe {
        esp_idf_svc::sys::esp_image_get_flash_size(esp_image_flash_size_t_ESP_IMAGE_FLASH_SIZE_16MB)
    };

    // let chip = unsafe {
    //     esp_idf_svc::sys::esp_chip_info(esp_image_flash_size_t_ESP_IMAGE_FLASH_SIZE_16MB)
    // };

    // 打印内存和外存的信息
    println!("Total memory: {} bytes", total_memory);
    println!("Free memory: {} bytes", free_memory);
    println!("Total PSRAM: {} bytes", total_psram);
    println!("Free PSRAM: {} bytes", free_psram);

    // 打印外部闪存的大小，单位为MB
    // println!("External flash size: {} MB", chip.size / 1024 / 1024);
}
