# Rust 嵌入式开发之 ESP32-S3

这是一个关于 ESP32-S3-R16N8 开发板学习案例，使用 Rust 语言的 STD 环境进行嵌入式开发。

![ESP32-S3-DevKitC-1](images/ESP32-S3-DevKitC-1.png)

注：

- 图片来源于网络。
- ESP32-S3-DevKitC-1 标记是 RGB@IO48 引脚。

## 示例目录

### 开门狗

- [代码禁用开门狗](app/wdg/code_disable_wdg/README.md)
- [SDK 配置禁用开门狗](app/wdg/sdkconfig_disable_wdg/README.md)

### 基础示例

- [Hello World](app/basic/hello_world/README.md)
- [日志级别](app/basic/log_level/README.md)
- [LED 闪烁](app/basic/blinky/README.md)

### 硬件

- [查看内存大小](app/hardware/hardware_memory_size/README.md)
- [RGB LED 闪烁](app/hardware/hardware_rgb_led/README.md)
- [RGB LED 灯珠闪烁](app/hardware/hardware_multiple_rgb_led/README.md)

### 延迟

- [FreeRtos 延迟](app/delay/freertos_delay/README.md)
- [定时器延迟](app/delay/async_timer_delay/README.md)

### ADC

- [ADC](app/adc/adc/README.md)
- [adc oneshot](app/adc/adc_oneshot/README.md)
- [3D 摇杆电位器](app/adc/joystick_potentiometer_3d/README.md)

3D joystick potentiometer

### 组建库

- [neopixel RGB LED](core/neopixel/README.md)

## 项目文档

- [Archlinux 环境搭建](./docs/Archlinux环境搭建.md)
- [esp-idf 环境配置](./docs/esp-idf环境配置.md)
- [问题答疑 Q&A](./docs/问题答疑Q&A.md)

## 参考文档

- [The Rust on ESP Book 简体中文版](https://narukara.github.io/rust-on-esp-book-zh-cn/introduction.html)
- [官方开发环境搭建文档](https://github.com/esp-rs/esp-idf-template/tree/master)
- [针对 RISC-V 和 Xtensa 目标](https://narukara.github.io/rust-on-esp-book-zh-cn/installation/riscv-and-xtensa.html)
- [esp-idf-sys 配置](https://github.com/esp-rs/esp-idf-sys/blob/master/BUILD-OPTIONS.md#sdkconfig)
- [Linux 和 macOS 平台工具链的标准设置](https://docs.espressif.com/projects/esp-idf/zh_CN/latest/esp32s3/get-started/linux-macos-setup.html)
- [查看模组的 PSRAM](https://espressif-docs.readthedocs-hosted.com/projects/esp-faq/zh-cn/latest/software-framework/storage/psram.html)
- [esp-idf C 绑定参考](https://esp-rs.github.io/esp-idf-svc/esp_idf_svc/index.html)
- [ESP32-S3 系列芯片介绍](https://blog.csdn.net/MJiarong_personal/article/details/121726585)
