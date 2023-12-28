# SPI 回环测试

注： 示例来源于官方。

## 引脚

- SCLK GPIO6
- SDI GPIO2
- SDO GPIO7
- CS_1 GPIO10
- CS_2 GPIO3
- 连接 SDI 和 SDO 引脚

## 执行指令

```shell
cargo run -r -p spi_loopback
```
