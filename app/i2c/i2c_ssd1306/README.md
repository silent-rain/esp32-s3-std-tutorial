# SSD13063 OLED 显示器闪烁黑白颜色

通过 I2C 协议控制一个 SSD13063 OLED 显示器，让它闪烁黑白两种颜色。

注： 示例来源于官方。

## 引脚

### OLED

- 正极: VCC 3.3v
- 负极: GND
- SDA: IO5
- SCL: IO6

## 执行指令

```shell
cargo run -r -p i2c_ssd1306
```
