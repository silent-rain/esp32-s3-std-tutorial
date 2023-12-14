# 定时器外部时钟

外部 32 kHz 晶振 - 可选 (XTAL32K)

XTAL32K_CLK 的时钟源可以是连接到 XTAL_32K_P 和 XTAL_32K_N 管脚的 32 kHz 晶振，也可以是外部电路生成的 32 kHZ 时钟信号。如果使用外部电路生成的时钟信号，该信号必须连接到 XTAL_32K_P 管脚。

注意：这是一个失败的示例

## 引脚

### 外部时钟源

- 信号: IO15

### 按钮

- 一端: GND
- 一端: IO5

## 执行指令

```shell
cargo run -r -p timer_external_clock
```
