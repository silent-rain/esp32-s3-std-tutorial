use esp_idf_svc::{
    hal::{
        delay::{FreeRtos, BLOCK},
        i2c::{I2cConfig, I2cDriver},
        peripherals::Peripherals,
        prelude::FromValueType,
    },
    log::EspLogger,
    sys::link_patches,
};

// SSD1306 的 I2C 地址
const SSD1306_ADDRESS: u8 = 0x3c;

fn main() -> anyhow::Result<()> {
    link_patches();
    // Bind the log crate to the ESP Logging facilities
    EspLogger::initialize_default();
    // 设置日志级别
    log::set_max_level(log::LevelFilter::Info);

    let peripherals = Peripherals::take()?;
    let i2c = peripherals.i2c0;
    let sda = peripherals.pins.gpio5;
    let scl = peripherals.pins.gpio6;

    println!("Starting I2C SSD1306 test");

    // 设置波特率为 100 kHz
    let config = I2cConfig::new().baudrate(100.kHz().into());
    let mut i2c = I2cDriver::new(i2c, sda, scl, &config)?;

    // 初始化显示器 - 不用担心这些字节的含义 - 它们是 SSD1306 特有的
    i2c.write(SSD1306_ADDRESS, &[0, 0xae], BLOCK)?; // 关闭显示器
    i2c.write(SSD1306_ADDRESS, &[0, 0xd4], BLOCK)?; // 设置显示时钟分频因子
    i2c.write(SSD1306_ADDRESS, &[0, 0x80], BLOCK)?; // 设置显示时钟分频因子为 128
    i2c.write(SSD1306_ADDRESS, &[0, 0xa8], BLOCK)?; // 设置多路复用比
    i2c.write(SSD1306_ADDRESS, &[0, 0x3f], BLOCK)?; // 设置多路复用比为 64
    i2c.write(SSD1306_ADDRESS, &[0, 0xd3], BLOCK)?; // 设置显示偏移量
    i2c.write(SSD1306_ADDRESS, &[0, 0x00], BLOCK)?; // 设置显示偏移量为 0
    i2c.write(SSD1306_ADDRESS, &[0, 0x40], BLOCK)?; // 设置显示起始行
    i2c.write(SSD1306_ADDRESS, &[0, 0x8d], BLOCK)?; // 设置电荷泵
    i2c.write(SSD1306_ADDRESS, &[0, 0x14], BLOCK)?; // 开启电荷泵
    i2c.write(SSD1306_ADDRESS, &[0, 0xa1], BLOCK)?; // 设置段重映射
    i2c.write(SSD1306_ADDRESS, &[0, 0xc8], BLOCK)?; // 设置行扫描方向
    i2c.write(SSD1306_ADDRESS, &[0, 0xda], BLOCK)?; // 设置 COM 引脚硬件配置
    i2c.write(SSD1306_ADDRESS, &[0, 0x12], BLOCK)?; // 设置 COM 引脚硬件配置为左右交替
    i2c.write(SSD1306_ADDRESS, &[0, 0x81], BLOCK)?; // 设置对比度
    i2c.write(SSD1306_ADDRESS, &[0, 0xcf], BLOCK)?; // 设置对比度为 207
    i2c.write(SSD1306_ADDRESS, &[0, 0xf1], BLOCK)?; // 设置预充电周期
    i2c.write(SSD1306_ADDRESS, &[0, 0xdb], BLOCK)?; // 设置 VCOMH 电压
    i2c.write(SSD1306_ADDRESS, &[0, 0x40], BLOCK)?; // 设置 VCOMH 电压为 0.77 * VCC
    i2c.write(SSD1306_ADDRESS, &[0, 0xa4], BLOCK)?; // 设置显示模式为正常显示
    i2c.write(SSD1306_ADDRESS, &[0, 0xa6], BLOCK)?; // 设置显示模式为正常显示
    i2c.write(SSD1306_ADDRESS, &[0, 0xaf], BLOCK)?; // 开启显示器
    i2c.write(SSD1306_ADDRESS, &[0, 0x20, 0x00], BLOCK)?; // 设置内存地址模式为水平地址模式，如

    // fill the display
    for _ in 0..64 {
        // 定义一个数组，包含 17 个字节，
        // 第一个字节是 0x40，表示后面的字节是数据，
        // 后面的 16 个字节都是 0xff，表示每个像素都是亮的
        let data: [u8; 17] = [
            0x40, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff,
        ];
        i2c.write(SSD1306_ADDRESS, &data, BLOCK)?;
    }

    // 使用一个无限循环，让显示器不断闪烁
    loop {
        // we are sleeping here to make sure the watchdog isn't triggered
        FreeRtos::delay_ms(500);
        // 设置显示模式为正常显示
        i2c.write(SSD1306_ADDRESS, &[0, 0xa6], BLOCK)?;
        FreeRtos::delay_ms(500);
        // 设置显示模式为反相显示
        i2c.write(SSD1306_ADDRESS, &[0, 0xa7], BLOCK)?;
    }
}
