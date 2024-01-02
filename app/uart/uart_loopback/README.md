# UART 回环测试

此示例通过 UART 传输数据。
连接 TX 和 RX 引脚，查看输出数据是否被读取为输入数据。

注： 示例来源于官方。

## 引脚

- TX GPIO12
- RX GPIO13

## 执行指令

```shell
cargo run -r -p uart_loopback
```
