//! # 查看 esp32-s3 内存以及外存大小
//!
//! ## 运行
//! ```shell
//! cargo run --bin hardware_memory_size
//! ```
#![allow(clippy::empty_loop)]

use esp_idf_svc::{
    hal::peripherals::Peripherals,
    log::EspLogger,
    sys::{
        heap_caps_get_free_size, heap_caps_get_total_size, heap_caps_malloc_extmem_enable,
        link_patches, MALLOC_CAP_8BIT, MALLOC_CAP_DEFAULT, MALLOC_CAP_INTERNAL,
        MALLOC_CAP_IRAM_8BIT, MALLOC_CAP_SPIRAM,
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

    // 启用外存
    unsafe { heap_caps_malloc_extmem_enable(1000) };

    // 获取内存的总大小和可用大小
    // 384 KB 内部 ROM
    let total_memory = unsafe { heap_caps_get_total_size(MALLOC_CAP_INTERNAL) };
    let free_memory = unsafe { heap_caps_get_free_size(MALLOC_CAP_INTERNAL) };
    println!("INTERNAL Total memory: {} bytes", total_memory);
    println!("INTERNAL Free memory: {} bytes", free_memory);

    // 片内 SRAM 512 KB = IRAM (192KB ） + DRAM（ 328KB )
    // TODO:获取失败
    let iram_total_memory = unsafe { heap_caps_get_total_size(MALLOC_CAP_IRAM_8BIT) };
    let iram_free_memory = unsafe { heap_caps_get_free_size(MALLOC_CAP_IRAM_8BIT) };
    let dram_total_memory = unsafe { heap_caps_get_total_size(MALLOC_CAP_8BIT) };
    let dram_free_memory = unsafe { heap_caps_get_free_size(MALLOC_CAP_8BIT) };
    println!("IRAM Total memory: {} bytes", iram_total_memory);
    println!("IRAM Free memory: {} bytes", iram_free_memory);
    println!("DRAM Total memory: {} bytes", dram_total_memory);
    println!("DRAM Free memory: {} bytes", dram_free_memory);

    // 获取外存SPIRAM的总大小和可用大小
    let total_psram = unsafe { heap_caps_get_total_size(MALLOC_CAP_SPIRAM) };
    let free_psram = unsafe { heap_caps_get_free_size(MALLOC_CAP_SPIRAM) };
    println!("SPIRAM Total PSRAM: {} bytes", total_psram);
    println!("SPIRAM Free PSRAM: {} bytes", free_psram);

    let total_memory = unsafe { heap_caps_get_total_size(MALLOC_CAP_DEFAULT) };
    let free_memory = unsafe { heap_caps_get_free_size(MALLOC_CAP_DEFAULT) };
    println!("DEFAULT memory: {} bytes", total_memory);
    println!("DEFAULT memory: {} bytes", free_memory);

    loop {}
}
