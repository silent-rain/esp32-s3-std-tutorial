# 按键中断计次 2

gpio_isr_register 注册 GPIO 中断处理程序，该处理程序是一个 ISR。处理程序将附加到运行此函数的同一 CPU 核心。

注意：这是一个失败的案例。

## 引脚

### 按钮

- 一端: GND
- 一端: IO5

## 执行指令

```shell
cargo run -r -p key_isr2
```
