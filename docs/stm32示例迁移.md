# Stm32 示例迁移

### DMA 数据转运

- [DMA 数据转运](./app/dma/dma_data_transfer)
- [DMA 数据连续转运](./app/dma/dma_data_continuous_transfer)
- [DMA+AD 多通道](./app/dma/scan_dma_and_ad_multichannel)
- [DMA+AD 多通道循环读取](./app/dma/scan_dma_and_ad_multichannel_loop)
- [DMA+AD 多通道分批读取](./app/dma/scan_dma_and_ad_multichannel_peek)

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
