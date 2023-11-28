# Archlinux 环境搭建

## 安装系统依赖

## 系统依赖

```shell
sudo pacman -S --needed gcc git make flex bison gperf python cmake ninja ccache dfu-util libusb
```

### 安装 Python

- 安装 Python 3.7 或更高版本；
- ESP-IDF 的要求；

```shell
sudo pacman --needed -S core/python
```

### 安装专注于异步 I/O 的多平台支持库

```shell
sudo pacman --needed -S extra/libuv
```

### Git

- ESP-IDF 的要求；

```shell
sudo pacman --needed -S git
```

## Rust nightly

虽然可以使用安装程序以 RISC-V Espressif SOC（esp32-cXX 和 esp32-hXX）为目标，但夜间 Rust 编译器和最近的库存 Clang 编译器（如 Clang 11+ 中）espup 也支持具有此架构的 SOC 。

```shell
rustup toolchain install nightly
```

## 安装 Cargo 子命令

```shell
cargo install cargo-generate
cargo install ldproxy
cargo install espup
cargo install espflash
cargo install cargo-espflash # Optional
```

### Cargo 子命令介绍

- cargo-generate
  - 使用一个预先存在的 git 仓库作为模板来创建一个新的 Rust 项目。
- ldproxy
  - 一个将链接参数转发给实际链接器的工具；
  - 实际链接器本身也是通过参数指定的。
- espup
  - 是一个用于安装和维护 Espressif Rust 生态系统所需的工具链的工具；
  - 它可以让您在 Rust 语言中为 Espressif 的 SoC 开发应用程序。
- espflash
  - 是一个用于给 Espressif 的 SoC 和模块刷写固件的工具；
  - 它基于 esptool.py，并支持 ESP32, ESP32-C2/C3/C6, ESP32-H2, ESP32-S2/S3, 和 ESP82664。
- cargo-espflash
  - 是一个用于给 Espressif 设备刷写固件的 cargo 扩展；
  - 它基于 espflash，并提供了一些方便的配置选项。

## 为 Espressif SoC 安装 Rust 和 Clang 工具链

### 安装工具链

为了启用对乐鑫目标的支持，espup 安装了以下工具：

- 乐鑫 Rust 分支，支持乐鑫目标
- nightly 工具链，支持 RISC-V 目标
- LLVM 分支，支持 Xtensa 目标
- GCC 工具链，用于链接最终的二进制文件

```shell
espup install
```

### 配置环境变量

```shell
cat $HOME/export-esp.sh >> ~/.bashrc
```

```shell
# 激活环境变量
source ~/.bashrc
```

## 构建/运行生成的项目：

```shell
# 编译项目, 自动使用合适的工具链和目标
cargo build
# 编译项目、向目标设备烧写程序、并开启一个串口监视器
cargo run
```

## flash 烧录

- 将上面的内容替换 dev/ttyUSB0 为您连接开发板的 USB 端口。如果您没有指定任何 USB 端口，espflash 将打印可识别的 USB 端口列表，供您选择所需的端口。
- 替换<your-project-name>为生成的项目名称
- 您可以--monitor 在 espflash 命令中包含参数以在刷新设备后打开串行监视器。
- 有关[espflash 使用的更多详细信息，请参阅自述文件](https://github.com/esp-rs/espflash/tree/main/espflash#usage)

```shell
espflash /dev/ttyUSB0 target/[xtensa-esp32-espidf|xtensa-esp32s2-espidf|xtensa-esp32s3-espidf|riscv32imc-esp-espidf]/debug/<your-project-name>

espflash /dev/ttyUSB0 target/xtensa-esp32s3-espidf/debug/<your-project-name>
```

## monitor

将上面的内容替换 dev/ttyUSB0 为您连接开发板的 USB 端口。
如果您没有指定任何 USB 端口，cargo-espflash/espflash 将打印已识别的 USB 端口列表，供您选择所需的端口。

```shell
espflash monitor /dev/ttyUSB0
```
