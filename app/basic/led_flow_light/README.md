# LED 流水灯

用 LED 实现流水灯的效果。
这里会使用到循环进行批量操作 LED 灯的状态，因此需要抹平每个引脚的类型，这里将定一个 ErasePinDriver trait 进行动态分发。

## 引脚

### LED 灯数组

- 正极: IO4,IO5,IO6...
- 负极: GND

## 执行指令

```shell
cargo run -r -p led_flow_light
```
