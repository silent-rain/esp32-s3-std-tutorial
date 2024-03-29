# LEDC 简单示例

LEDC 是 LED 控制器的缩写，它是 ESP32-S3 的一个外设，主要用于控制 LED 的亮度，也可以用于生成 PWM 信号用于其他用途。它有 8 个通道，可以生成独立的波形，例如用于驱动 RGB LED 设备。PWM 控制器可以自动增加或减少占空比，实现无需处理器干预的渐变效果。

它没有单独的 PWM 模块，因为 LEDC 模块已经提供了 PWM 的功能。

要使用 LEDC，您需要进行以下几个步骤：

- 定时器配置，指定 PWM 信号的频率和占空比分辨率。
- 通道配置，将其与定时器和 GPIO 管脚关联，输出 PWM 信号。
- 改变 PWM 信号，改变输出的占空比，从而改变 LED 的亮度。这可以完全由软件控制，也可以使用硬件渐变功能。

注： 示例来源于官方。

## 引脚

### LED 灯

- 正极: IO5
- 负极: GND

## 执行指令

```shell
cargo run -r -p ledc_simple
```
