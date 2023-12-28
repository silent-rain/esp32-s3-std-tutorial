# ST7789 LCD 显示屏

使用 SPI 协议来驱动一个 ST7789 的 LCD 显示屏，并在上面显示一张螃蟹的图片。

注： 示例来源于官方。
注： 这是一个失败的示例, `display-interface-spi` 依赖库下载失败。

## 引脚

- RST GPIO3
- DC GPIO4
- BACKLIGHT GPIO5
- SCLK GPIO6
- SDA GPIO7

## 执行指令

```shell
cargo run -r -p spi_st7789
```
