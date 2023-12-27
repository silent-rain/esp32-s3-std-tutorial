# DMA 介绍

注：当前的 `esp-idf-sys = "0.33.7"` 库未包含 DMA 的绑定。

ESP32-S3 是一款由 Espressif Systems 开发的高性能、低功耗的 Wi-Fi 和蓝牙 SoC 芯片。它内置了 DMA（直接内存访问）引擎，用于实现异步的内存传输操作。DMA 引擎可以将内存传输操作从 CPU 中卸载出来，从而实现更高效和更快速的数据传输。
ESP32-S3 的 DMA 引擎支持多个通道，每个通道可以独立地执行内存传输操作。这意味着可以同时进行多个内存传输操作，而不会相互干扰。DMA 引擎还支持不同的传输模式，包括内存到内存、内存到外设、外设到内存等。
在 ESP-IDF（Espressif IoT Development Framework）中，提供了一套 API 用于配置和操作 ESP32-S3 的 DMA 引擎。通过这些 API，可以设置 DMA 通道的参数，如传输模式、传输方向、传输大小等。还可以配置 DMA 引擎的中断和回调函数，以便在传输完成时进行相应的处理。
使用 ESP32-S3 的 DMA 引擎可以带来多方面的好处。首先，它可以减轻 CPU 的负担，将内存传输操作交给 DMA 引擎处理，从而释放 CPU 的计算资源。其次，DMA 引擎可以实现高速的数据传输，提高系统的响应速度和效率。此外，DMA 引擎还可以支持数据的连续传输，减少了 CPU 的干预，提高了系统的并发性能。

ESP32-S3 芯片具有 DMA（直接内存访问）引擎，可以进行异步内存复制操作。这意味着 DMA 引擎可以将内存复制操作从 CPU 中卸载，从而实现更高效和更快速的数据传输。
ESP-IDF（Espressif 物联网开发框架）提供了一个异步 memcpy API，它封装了所有 DMA 配置和操作。该 API 类似于标准的 libc memcpy 函数。它允许在第一个内存复制完成之前排队多个内存复制请求，实现计算和内存复制的重叠。
要在 ESP32-S3 上使用异步 memcpy API，您需要配置和安装异步 memcpy 驱动程序。根据底层 DMA 引擎的不同，安装驱动程序的方法也不同。例如，您可以使用 esp_async_memcpy_install_gdma_ahb()函数基于 AHB GDMA 引擎安装驱动程序。
安装驱动程序后，您可以使用 esp_async_memcpy()函数将内存复制请求发送到 DMA 引擎。该函数接受目标地址、源地址和要复制的字节数作为参数。您还可以选择性地提供一个回调函数，在内存复制完成时执行。
要卸载异步 memcpy 驱动程序，可以使用 esp_async_memcpy_uninstall()函数。
总的来说，ESP32-S3 上的 DMA 引擎提供了一种将内存复制操作从 CPU 中卸载的方法，提高了性能和效率。

## 参考文档

- [内存同步](https://docs.espressif.com/projects/esp-idf/zh_CN/latest/esp32s3/api-reference/system/mm_sync.html?highlight=dma)
- [异步内存复制](https://docs.espressif.com/projects/esp-idf/zh_CN/latest/esp32s3/api-reference/system/async_memcpy.html?highlight=dma)
- [模数转换器 (ADC) 单次转换模式驱动](https://docs.espressif.com/projects/esp-idf/zh_CN/latest/esp32s3/api-reference/peripherals/adc_oneshot.html?highlight=dma)
- [模数转换器 (ADC) 连续转换模式驱动](https://docs.espressif.com/projects/esp-idf/zh_CN/latest/esp32s3/api-reference/peripherals/adc_continuous.html?highlight=dma)
