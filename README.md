# Rust 嵌入式开发之 ESP32-S3

这是一个关于 ESP32-S3-R16N8 开发板学习案例，使用 Rust 语言的 STD 环境进行嵌入式开发。

## 示例目录

### 基础示例

- [代码禁用开门狗](src/bin/basic/code_disable_wdg.rs)
- [SDK 配置禁用开门狗](src/bin/basic/sdkconfig_disable_wdg.rs)
- [Hello World](src/bin/basic/hello_world.rs)

## 硬件

- [查看内存大小](src/bin/hardware/hardware_memory_size.rs)

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
