# RMT 收发器

一个设置 RMT 发送器和 RMT 接收器的简单示例

GPIO 引脚 2 为输入，GPIO 引脚 4 为输出

TYPICAL OUTPUT:
Tx Loop
Rx Loop
level0 = High dur0 = PulseTicks(620) level1 = Low dur1 = PulseTicks(620)
level0 = High dur0 = PulseTicks(620) level1 = Low dur1 = PulseTicks(620)
level0 = High dur0 = PulseTicks(210) level1 = Low dur1 = PulseTicks(410)
level0 = High dur0 = PulseTicks(410) level1 = Low dur1 = PulseTicks(210)
level0 = High dur0 = PulseTicks(210) level1 = Low dur1 = PulseTicks(0)
Rx Loop
level0 = High dur0 = PulseTicks(620) level1 = Low dur1 = PulseTicks(620)
level0 = High dur0 = PulseTicks(620) level1 = Low dur1 = PulseTicks(620)
level0 = High dur0 = PulseTicks(210) level1 = Low dur1 = PulseTicks(410)
level0 = High dur0 = PulseTicks(410) level1 = Low dur1 = PulseTicks(210)
level0 = High dur0 = PulseTicks(210) level1 = Low dur1 = PulseTicks(0)
Tx Loop

注：这是官方示例。

## 引脚

### RMT 收发器

- 输入: IO2
- 输出: IO4

## 执行指令

```shell
cargo run -r -p rmt_transceiver
```
