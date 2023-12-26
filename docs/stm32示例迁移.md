# Stm32 示例迁移

### DMA 数据转运

- [打印内存地址](./app/dma/print_memory_address)
- [DMA 数据转运](./app/dma/dma_data_transfer)
- [DMA 数据连续转运](./app/dma/dma_data_continuous_transfer)
- [DMA+AD 多通道](./app/dma/scan_dma_and_ad_multichannel)
- [DMA+AD 多通道循环读取](./app/dma/scan_dma_and_ad_multichannel_loop)
- [DMA+AD 多通道分批读取](./app/dma/scan_dma_and_ad_multichannel_peek)

### USART 串行接口

- [串行接口配置](./app/usart/serial_config)
- [串行接口发送与接收](./app/usart/serial_tx_and_rx)
- [串行接口重新配置](./app/usart/serial_reconfigure)
- [串行接口写入格式化字符串](./app/usart/serial_fmt)
- [串行接口连续发送与接收](./app/usart/serial_continuous_tx_and_rx)
- [串行接口中断](./app/usart/serial_interrupt_idle)
- [串行接口收发 HEX 数据包](./app/usart/serial_hex_packet)
- [串行接口收发文本数据包](./app/usart/serial_text_packet)

### I2C 通信

- [I2C MPU6050 crate 读写](./app/i2c/i2c_mpu6050_crate)

### SPI 通信

- [SPI 软件读写 W25Q64](./app/spi/spi_soft_w25q64)
- [SPI 硬件读写 W25Q64](./app/spi/spi_hard_w25q64)
- [w25q crate 读写 W25Q64](./app/spi/spi_w25q_crate)

### RTC

- [RTC 实时时钟计数器](./app/rtc/rtc_counter)
- [BKP 断电恢复](./app/rtc/rtc_bkp)
- [读写备份寄存器](./app/rtc/rtc_bkp_dyn_data)
- [RTC 告警闪烁 LED](./app/rtc/rtc_alarm_blinky)
- [RTC 实时时间](./app/rtc/rtc_time)

### PWR 电源控制

- [修改系统时钟主频](./app/pwr/syst_freq)
- [睡眠模式-串口发送接收](./app/pwr/sleep_mode_serial_tx_and_rx)
- [停止模式-对射式红外传感器计次](./app/pwr/stop_mode_infrared_sensor_count)
- [待机模式-实时时钟计数](./app/pwr/standby_mode_rtc_counter)

### WDG 看门狗

- [独立看门狗](./app/wdg/iwdg)
- [窗口看门狗](./app/wdg/wwdg)

### FLASH

- [读写内部 FLASH](./app/flash/internal_flash)
- [读取芯片 ID](./app/flash/read_chip_id)

### 常用外设工具库封装

- [硬件工具库](./core/hardware)
  pub mod flash_store;
  pub mod key;
  pub mod serial;
  pub mod w25q64;
