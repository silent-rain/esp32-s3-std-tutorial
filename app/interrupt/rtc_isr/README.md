# RTC 中断

RTC 中断是一种用于定时或唤醒的低功耗中断。ESP32 的 RTC 模块包括一个 64 位计数器，一个 8 位分频器，一个 64 位闹钟寄存器，一个 64 位时间戳寄存器，一个 32 位中断状态寄存器，一个 32 位中断使能寄存器，和一个 32 位中断清除寄存器。

## 执行指令

```shell
cargo run -r -p rtc_isr
```
