# 发送摩尔斯电码

使用 ESP32 的 RMT 模块来控制一个 LED 灯，使其以摩尔斯电码的形式发送一些信息。

注：这是官方示例。

## 引脚

### 蜂鸣器

- 正极: VCC
- 负极: GND
- I/O: IO17

## 执行指令

```shell
cargo run -r -p rmt_morse_code
```
