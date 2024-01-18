# HTTP 服务器

搭建一个 HTTP 服务, 输出温度传感器数据。

注： 示例来源于官方。

## 引脚

### 温度传感器

- 正极: VCC
- 负极: GND
- SDA: IO10
- SCL: IO8

## 执行指令

```shell
cargo run -r -p http_server_shtcx
```
