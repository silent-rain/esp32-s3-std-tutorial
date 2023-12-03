# AI 协助

## 提供示例

```text
我希望您担任使用 Rust 编程语言进行 EPS32-S3 单片机的嵌入式开发专家。
请指引我使用 esp-idf-svc 框架在 Rust Std 环境中进行嵌入式开发。
请提供一个读取 ESP32-S3-N16R8（8MB PSRAM + 16MB FLASH）的开发板的 PSRAM 和 FLASH 内存大小的示例，请确保给出的示例的正确性。
```

## 提供示例 2

```text
我希望您担任以 Rust 编程语言进行 EPS32-S3 单片机的嵌入式开发专家。
请指根据三个引号包裹内的要求，使用 esp-idf-svc 框架在 Rust Std 环境中进行嵌入式开发，并确保提供代码的正确性。
以下代码读取EPS32-S3-R16N8 的内存不正确，请帮我分析原因并提供解决方案
”“”
code:
// 获取内存的总大小和可用大小
let total_memory = unsafe { heap_caps_get_total_size(MALLOC_CAP_INTERNAL) };
let free_memory = unsafe { heap_caps_get_free_size(MALLOC_CAP_INTERNAL) };

// 获取外存的总大小和可用大小
let total_psram = unsafe { heap_caps_get_total_size(MALLOC_CAP_SPIRAM) };
let free_psram = unsafe { heap_caps_get_free_size(MALLOC_CAP_SPIRAM) };
// unsafe { heap_caps_malloc_extmem_enable() };

// 打印内存和外存的信息
println!("Total memory: {} bytes", total_memory);
println!("Free memory: {} bytes", free_memory);
println!("Total PSRAM: {} bytes", total_psram);
println!("Free PSRAM: {} bytes", free_psram);


output:
Total memory: 405604 bytes
Free memory: 382952 bytes
Total PSRAM: 0 bytes
Free PSRAM: 0 bytes
“”“
```

## 代码转换

```
我希望您担任以 Rust 编程语言进行 EPS32-S3 单片机的嵌入式开发专家。
请指根据三个引号包裹内的要求，使用 esp-idf-svc 框架在 Rust Std 环境中进行嵌入式开发，请将以下代码转换为rust代码，并确保提供代码的正确性。

"""
import esp
eso.flash_size()/1024/1024


import micropython
micropython.mem_info()
"""
```

## 补充 todo 示例

```
我希望你担任 Rust 编程语言嵌入式开发专家。指引我使用 stm32f1xx-hal="0.10.0" 嵌入式框架进行开发学习，请确保给出的示例的正确性。
以下是 RTC 告警中断的示例，请帮我补充 todo 信息，并确保正确。
```
