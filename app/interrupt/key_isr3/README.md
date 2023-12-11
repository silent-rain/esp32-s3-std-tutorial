# 按键中断计次 3

安装 GPIO 驱动程序的 ETS_GPIO_INTR_SOURCE ISR 处理程序服务，该服务允许每个引脚 GPIO 中断处理程序。

该函数与 gpio_isr_register() 不兼容 - 如果使用该函数，将为所有 GPIO 中断注册一个全局 ISR 。如果使用此函数，ISR 服务将提供全局 GPIO ISR，并且通过 gpio_isr_handler_add() 函数注册各个引脚处理程序。

## 引脚

### 按钮

- 一端: GND
- 一端: IO5

## 执行指令

```shell
cargo run -r -p key_isr3
```
