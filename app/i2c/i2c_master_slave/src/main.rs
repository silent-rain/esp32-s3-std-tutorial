use esp_idf_svc::{
    hal::{
        delay::BLOCK,
        gpio::AnyIOPin,
        i2c::{I2c, I2cConfig, I2cDriver, I2cSlaveConfig, I2cSlaveDriver},
        peripheral::Peripheral,
        peripherals::Peripherals,
        prelude::FromValueType,
        units::Hertz,
    },
    log::EspLogger,
    sys::link_patches,
};

// 从机的 I2C 地址
const SLAVE_ADDR: u8 = 0x22;
// 从机的缓冲区大小
const SLAVE_BUFFER_SIZE: usize = 128;

fn main() -> anyhow::Result<()> {
    link_patches();
    // Bind the log crate to the ESP Logging facilities
    EspLogger::initialize_default();
    // 设置日志级别
    log::set_max_level(log::LevelFilter::Info);

    // Get the peripherals
    let peripherals = Peripherals::take()?;

    let mut i2c_master = i2c_master_init(
        peripherals.i2c0,
        peripherals.pins.gpio20.into(),
        peripherals.pins.gpio21.into(),
        100.kHz().into(),
    )?;

    let mut i2c_slave = i2c_slave_init(
        peripherals.i2c1,
        peripherals.pins.gpio18.into(),
        peripherals.pins.gpio19.into(),
        SLAVE_BUFFER_SIZE,
        SLAVE_ADDR,
    )?;

    // 定义一个数组，包含 8 个字节，用于发送数据
    let tx_buf: [u8; 8] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];

    println!("-------- 开始测试简单的主机写入 --------");
    // 向从机发送 tx_buf 中的数据，使用阻塞模式
    i2c_master.write(SLAVE_ADDR, &tx_buf, BLOCK)?;

    // 定义一个可变的数组，包含 8 个字节，用于接收数据
    let mut rx_buf: [u8; 8] = [0; 8];
    // 从主机读取数据到 rx_buf 中，使用阻塞模式
    match i2c_slave.read(&mut rx_buf, BLOCK) {
        Ok(_) => println!("Master send {:?} Slave receives {:?}", tx_buf, rx_buf),
        Err(e) => println!("Error: {:?}", e),
    }

    println!("-------- 开始测试简单的主机读取 --------");
    // 向主机发送 tx_buf 中的数据，使用阻塞模式
    // 在主机尝试读取之前，先填充从机的缓冲区
    i2c_slave.write(&tx_buf, BLOCK)?;

    // 定义一个可变的数组，包含 8 个字节，用于接收数据
    let mut rx_buf: [u8; 8] = [0; 8];
    // 从从机读取数据到 rx_buf 中，使用阻塞模式
    match i2c_master.read(SLAVE_ADDR, &mut rx_buf, BLOCK) {
        Ok(_) => println!("Slave send {:?} Master receives {:?}", tx_buf, rx_buf),
        Err(e) => println!("Error: {:?}", e),
    }

    println!("-------- 开始测试读写寄存器 --------");

    let thread0 = std::thread::Builder::new()
        .stack_size(7000) // 设置线程的栈大小为 7000 字节
        .spawn(move || {
            let mut data: [u8; 256] = [0; 256];
            loop {
                let mut reg_addr: [u8; 1] = [0];
                // 从主机读取寄存器的数据到 reg_addr 中，使用非阻塞模式
                let res = i2c_slave.read(&mut reg_addr, BLOCK);
                if res.is_err() {
                    println!(
                        "SLAVE: failed to read register address from master: Error: {:?}",
                        res
                    );
                    continue;
                }
                let mut rx_data: [u8; 1] = [0];
                // 从主机读取寄存器的数据到 rx_data 中，使用非阻塞模式
                match i2c_slave.read(&mut rx_data, 0) {
                    Ok(_) => {
                        // 从机写入操作，并显示寄存器的地址和数据
                        println!(
                            "SLAVE: write operation {:#04x} to reg addr {:#04x}",
                            rx_data[0], reg_addr[0]
                        );
                        // 将 rx_data[0] 的值赋给 data 数组中对应寄存器地址的元素
                        data[reg_addr[0] as usize] = rx_data[0];
                    }
                    Err(_) => {
                        let d = data[reg_addr[0] as usize];
                        // 从机读取操作，并显示寄存器的地址和数据
                        println!(
                            "SLAVE: read operation {:#04x} from reg addr {:#04x}",
                            d, reg_addr[0]
                        );
                        // 向主机发送 d 的值，使用阻塞模式
                        i2c_slave.write(&[d], BLOCK).unwrap();
                    }
                }
            }
        })?;

    // allow thread to run
    // 让出当前线程的执行权，让其他线程有机会运行
    std::thread::yield_now();

    // 表示要读写的寄存器的地址
    let reg_addr: u8 = 0x05;
    // 表示要写入的新值
    let new_value: u8 = 0x42;

    // 表示主机读取寄存器的地址
    println!("MASTER: read reg addr {:#04x}", reg_addr);

    // 定义一个可变的数组，包含 1 个字节，用于接收数据
    let mut rx_buf: [u8; 1] = [0; 1];
    // TODO: make write_read work
    // 向从机发送寄存器的地址，使用阻塞模式
    i2c_master.write(SLAVE_ADDR, &[reg_addr], BLOCK)?;
    // 从从机读取寄存器的数据到 rx_buf 中，使用阻塞模式
    i2c_master.read(SLAVE_ADDR, &mut rx_buf, BLOCK)?;
    println!(
        "MASTER: value of reg addr {:#04x} is {:#04x}",
        reg_addr, rx_buf[0]
    );

    println!("---------------------");

    // 表示主机写入新值到寄存器的地址
    println!(
        "MASTER: write {:#04x} to reg addr {:#04x}",
        new_value, reg_addr
    );
    // 向从机发送寄存器的地址和新值，使用阻塞模式
    i2c_master.write(SLAVE_ADDR, &[reg_addr, new_value], BLOCK)?;

    println!("---------------------");

    println!("MASTER: read reg addr {:#04x}", reg_addr);
    // 定义一个可变的数组，包含 1 个字节，用于接收数据
    let mut rx_buf: [u8; 1] = [0; 1];
    // TODO: make write_read work
    // 向从机发送寄存器的地址，使用阻塞模式
    i2c_master.write(SLAVE_ADDR, &[reg_addr], BLOCK)?;
    // 从从机读取寄存器的数据到 rx_buf 中，使用阻塞模式
    i2c_master.read(SLAVE_ADDR, &mut rx_buf, BLOCK)?;
    println!(
        "MASTER: value of reg addr {:#04x} is {:#04x}",
        reg_addr, rx_buf[0]
    );

    // 等待线程结束
    thread0.join().unwrap();
    Ok(())
}

/// 用于初始化主机的 I2C 驱动
fn i2c_master_init<'d>(
    i2c: impl Peripheral<P = impl I2c> + 'd,
    sda: AnyIOPin,
    scl: AnyIOPin,
    baudrate: Hertz, // 波特率
) -> anyhow::Result<I2cDriver<'d>> {
    let config = I2cConfig::new().baudrate(baudrate);
    let driver = I2cDriver::new(i2c, sda, scl, &config)?;
    Ok(driver)
}

/// 用于初始化从机的 I2C 驱动
fn i2c_slave_init<'d>(
    i2c: impl Peripheral<P = impl I2c> + 'd,
    sda: AnyIOPin,
    scl: AnyIOPin,
    buflen: usize,  // 缓冲区的长度
    slave_addr: u8, // 从机的地址
) -> anyhow::Result<I2cSlaveDriver<'d>> {
    let config = I2cSlaveConfig::new()
        .rx_buffer_length(buflen) // 设置接收缓冲区的长度为参数给定的值
        .tx_buffer_length(buflen); // 设置发送缓冲区的长度为参数给定的值
    let driver = I2cSlaveDriver::new(i2c, sda, scl, slave_addr, &config)?;
    Ok(driver)
}
