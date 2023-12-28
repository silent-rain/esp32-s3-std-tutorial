# SPI 异步回环测试

注： 示例来源于官方。
注： 这是一个失败的示例，SDK 配置没有生效。

## 配置 SDK

```text
# SPI Configuration
#
# CONFIG_SPI_MASTER_IN_IRAM is not set
CONFIG_SPI_MASTER_ISR_IN_IRAM=y
# CONFIG_SPI_SLAVE_IN_IRAM is not set
CONFIG_SPI_SLAVE_ISR_IN_IRAM=y
# end of SPI Configuration

# ============= 调整为以下配置 =============

# SPI Configuration
#
# CONFIG_SPI_MASTER_IN_IRAM is not set
# CONFIG_SPI_MASTER_ISR_IN_IRAM is not set
# CONFIG_SPI_SLAVE_IN_IRAM is not set
# CONFIG_SPI_SLAVE_ISR_IN_IRAM is not set
# end of SPI Configuration
```

## 引脚

- SCLK GPIO6
- SDI GPIO2
- SDO GPIO7
- CS_1 GPIO10
- CS_2 GPIO3
- 连接 SDI 和 SDO 引脚

## 执行指令

```shell
cargo run -r -p spi_loopback_async
```
