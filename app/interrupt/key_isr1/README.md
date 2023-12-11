# 按键中断计次 1

subscribe 函数中断，在收到中断通知后，需要在非中断上下文中，再次调用 PinDriver::enable_interrupt 方法，来重新启用中断，否则会触发看门狗定时器。

## 引脚

### 按钮

- 一端: GND
- 一端: IO5

## 执行指令

```shell
cargo run -r -p key_isr1
```
