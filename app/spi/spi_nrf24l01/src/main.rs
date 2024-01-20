use esp32s3_nrf24l01::{
    self, Configuration, CrcMode, DataRate, Payload, RxMode, StandbyMode, TxMode, NRF24L01,
};

use esp_idf_svc::{
    hal::{
        delay::FreeRtos,
        gpio::{AnyIOPin, InputPin, OutputPin},
        peripheral::Peripheral,
        peripherals::Peripherals,
        spi::SPI2,
    },
    log::EspLogger,
    sys::{link_patches, EspError},
};

/// RF24L01 发送协议地址
pub const NRF24L01_TX_ADDR: &[u8] = b"fnord";
/// RF24L01 接收协议地址
pub const NRF24L01_RX_ADDR: &[u8] = b"fnord";
/// RF24L01 接收通道
pub const NRF24L01_RX_ADDR_P0: usize = 0x00;
pub const NRF24L01_RX_ADDR_P1: usize = 0x01;
pub const NRF24L01_RX_ADDR_P2: usize = 0x02;
pub const NRF24L01_RX_ADDR_P3: usize = 0x03;
pub const NRF24L01_RX_ADDR_P4: usize = 0x04;
pub const NRF24L01_RX_ADDR_P5: usize = 0x05;

type Device<'d> = NRF24L01<'d, EspError>;

pub type RxTY<'d> = RxMode<Device<'d>>;

/// NRF24L01 传输指令
#[derive(Debug)]
pub struct NRF24L01Cmd {
    pub cmd: i32,
}

/// NRF24L01 2.4G 无线通信
pub struct Nrf24L01<'d> {
    pub nrf24: StandbyMode<Device<'d>>,
}

impl<'d> Nrf24L01<'d> {
    /// 初始化 NRF24L01 SPI 2.4 GHz 无线通信
    pub fn new(
        spi2: impl Peripheral<P = SPI2> + 'd,
        sclk: impl Peripheral<P = impl OutputPin> + 'd,
        miso: Option<impl Peripheral<P = impl InputPin + OutputPin> + 'd>,
        mosi: impl Peripheral<P = impl OutputPin> + 'd,
        cs: Option<impl Peripheral<P = impl OutputPin> + 'd>,
        ce: AnyIOPin,
        csn: AnyIOPin,
    ) -> Self {
        let nrf24 = NRF24L01::new(spi2, sclk, miso, mosi, cs, ce, csn).unwrap();

        let mut nrf24l01 = Nrf24L01 { nrf24 };

        // 配置设备
        nrf24l01.init_config();

        nrf24l01
    }

    /// 配置设备
    fn init_config(&mut self) {
        // 设置频率为2.476 GHz
        // self.nrf24.set_frequency(76).unwrap();

        // 设置 nRF24 无线模块的通信速率为 2 Mbps，输出功率为 -18 dBm
        // RF output power in TX mode
        // * `00`: -18 dBm
        // * `01`: -12 dBm
        // * `10`: -6 dBm
        // * `11`: 0 dBm
        self.nrf24.set_rf(&DataRate::R250Kbps, 00).unwrap();
        // 关闭自动重传功能
        self.nrf24.set_auto_retransmit(0, 0).unwrap();
        // 设置CRC模式
        self.nrf24.set_crc(CrcMode::Disabled).unwrap();
        // 自动应答功能
        self.nrf24.set_auto_ack(&[true; 6]).unwrap();

        // 设置地址的长度，它可以是3，4或5字节
        // self.nrf24.set_pipes_rx_lengths(lengths);

        // 设置接收地址
        self.nrf24
            .set_rx_addr(NRF24L01_RX_ADDR_P0, NRF24L01_RX_ADDR)
            .unwrap();

        // 配置要启用或禁用接收管道
        // NRF24L01一共有6个管道，分别是0到5。
        // 每个管道都有一个5字节的地址，用来识别发送和接收的数据包。
        // 默认使用的是管道0
        self.nrf24.set_pipes_rx_enable(&[true; 6]).unwrap();

        // 设置发送地址
        self.nrf24.set_tx_addr(NRF24L01_TX_ADDR).unwrap();

        // 清空发送缓冲区
        self.nrf24.flush_tx().unwrap();
        // 清空接收缓冲区
        self.nrf24.flush_rx().unwrap();
    }

    /// 接收数据转换为字符串
    /// 最大长度: 32
    pub fn payload_string(payload: Payload) -> String {
        let mut s = String::new();
        let data = payload.as_ref();
        for byte in data {
            s.push(*byte as char);
        }
        s
    }

    /// 接收数据包并返回有效载荷
    pub fn recv_data(rx: &mut RxMode<Device>) -> Option<Payload> {
        // 是否有数据包到达
        let _pipe = match rx.can_read() {
            Ok(v) => v,
            Err(_err) => return None,
        };
        // 接收数据包
        let payload = rx.read().unwrap();
        // let data: &[u8]  = payload.as_ref();
        // 处理接收到的数据包
        // println!("Received {} bytes on pipe {}", payload.len(), pipe);
        Some(payload)
    }

    /// 发送数据
    pub fn send_data(tx: &mut TxMode<Device>, bytes: &[u8]) {
        // 发送数据
        tx.send(bytes).expect("Failed to send data");

        // 等待队列清空
        while !tx.can_send().unwrap() {}

        // 清空发送缓冲区
        // tx.flush_tx().unwrap();

        // 等待队列清空
        // tx.wait_empty().unwrap();
    }
}

fn main() -> anyhow::Result<()> {
    link_patches();
    // Bind the log crate to the ESP Logging facilities
    EspLogger::initialize_default();
    // 设置日志级别
    log::set_max_level(log::LevelFilter::Info);

    // Get the peripherals
    let peripherals = Peripherals::take()?;

    let spi2 = peripherals.spi2;

    let sclk = peripherals.pins.gpio4;
    let miso = peripherals.pins.gpio5;
    let mosi = peripherals.pins.gpio6;
    let ce = peripherals.pins.gpio7;
    let csn = peripherals.pins.gpio8;
    let cs = peripherals.pins.gpio9;

    // 初始化 NRF24L01 2.4 GHz 无线通信
    let nrf24l01 = Nrf24L01::new(
        spi2,
        sclk,
        Some(miso),
        mosi,
        Some(cs),
        ce.into(),
        csn.into(),
    );
    // let mut tx = nrf24l01.nrf24.tx().unwrap();
    // println!("init nrf24l01 tx ...");

    let mut rx = nrf24l01.nrf24.rx().unwrap();
    println!("init nrf24l01_rx ...");

    log::info!("loop");
    loop {
        // let bytes = "test".as_bytes();
        // Nrf24L01::send_data(&mut tx, bytes);
        // log::info!("send");

        let payload = Nrf24L01::recv_data(&mut rx).unwrap();
        let data = Nrf24L01::payload_string(payload);
        println!("{:?}", data.as_str());
        FreeRtos::delay_ms(1000);
    }
}
