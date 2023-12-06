# 配置 SDK

## Flash 配置

```
Serial flasher config  --->
```

- 配置模式

```
Flash SPI mode (DIO)  ---> QIO
```

- 配置频率

```
Flash SPI speed (80 MHz)  ---> 80 MHz
```

- 设置内存大小

```
Flash size (2 MB)  ---> 16 MB
```

- 自动检测, 闪烁引导加载程序时检测闪存大小

```
[*] Detect flash size when flashing bootloader
```

## ESP PSRAM 配置

```
Component config  ---> ESP PSRAM  --->
```

- 启用 PSRAM

```
[*] Support for external, SPI-connected RAM
```

### RAM 配置

```
SPI RAM config  --->
```

- (QUAD/OCT) of SPI RAM chip in use (Octal Mode PSRAM) 设置为 Octal Mode PSRAM，因为 ESP32 S3 N16R8 模块内置的 PSRAM 是八线模式的 PSRAM

```
Mode (QUAD/OCT) of SPI RAM chip in use (Quad Mode PSRAM)  ---> Octal Mode PSRAM
```

- 设置 PSRAM 的频率（可选）

```
Set RAM clock speed (80MHz clock speed)  ---> 80MHz clock speed
```

## 开门狗配置

- 禁用开门狗

```
(Top) → Component config → ESP System Settings

# 关闭中断看门狗定时器 (IWDT)
[ ] Interrupt watchdog
# 任务看门狗定时器 (TWDT)
[ ] Enable Task Watchdog Timer
```

