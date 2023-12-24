use esp_idf_svc::{
    hal::{
        delay::FreeRtos,
        peripherals::Peripherals,
        rmt::{
            FixedLengthSignal, PinState, Pulse, PulseTicks, Receive, RmtReceiveConfig,
            RmtTransmitConfig, RxRmtDriver, TxRmtDriver,
        },
    },
    log::EspLogger,
    sys::link_patches,
};

fn main() -> anyhow::Result<()> {
    link_patches();
    EspLogger::initialize_default();
    log::set_max_level(log::LevelFilter::Info);

    println!("Starting APP!");

    let peripherals = Peripherals::take()?;

    /*
     *********************** 设置RMT接收器 ******************************
     */
    // 设置空闲阈值 700u16
    let receive_config = RmtReceiveConfig::new().idle_threshold(700u16);
    let mut rx = RxRmtDriver::new(
        peripherals.rmt.channel2,
        peripherals.pins.gpio4,
        &receive_config,
        250, // 环形缓冲区大小
    )?;

    // 开始接收
    rx.start()?;

    let _ = std::thread::Builder::new()
        .stack_size(10000)
        .spawn(move || loop {
            println!("Rx Loop");

            let mut pulses = [(Pulse::zero(), Pulse::zero()); 250];

            // 请参阅 sdkconfig.defaults 以确定刻度时间值（默认值为 tick=10毫秒）
            // 将 ticks_to_wait 设置为0表示非阻塞
            let receive = rx.receive(&mut pulses, 0).unwrap();

            if let Receive::Read(length) = receive {
                let pulses = &pulses[..length];

                for (pulse0, pulse1) in pulses {
                    println!("0={pulse0:?}, 1={pulse1:?}");
                }
            }

            FreeRtos::delay_ms(500);
        });

    /*
     *********************** 设置RMT发送器 ******************************
     */

    // 准备 tx_config
    // 默认使用一个内存块或64个信号，时钟分频器设置为80（1us tick）
    let mut tx = TxRmtDriver::new(
        peripherals.rmt.channel0,
        peripherals.pins.gpio5,
        &RmtTransmitConfig::new(),
    )?;

    // 准备要发送的信号脉冲信号。
    let one_low = Pulse::new(PinState::Low, PulseTicks::new(410)?);
    let one_high = Pulse::new(PinState::High, PulseTicks::new(210)?);
    let zero_low = Pulse::new(PinState::Low, PulseTicks::new(210)?);
    let zero_high = Pulse::new(PinState::High, PulseTicks::new(410)?);
    let sync_low = Pulse::new(PinState::Low, PulseTicks::new(620)?);
    let sync_high = Pulse::new(PinState::High, PulseTicks::new(620)?);

    let _ = std::thread::spawn(move || loop {
        println!("Tx Loop");

        // 创建序列
        let mut signal = FixedLengthSignal::<5>::new();
        signal.set(0, &(sync_high, sync_low)).unwrap();
        signal.set(1, &(sync_high, sync_low)).unwrap();
        signal.set(2, &(one_high, one_low)).unwrap();
        signal.set(3, &(zero_high, zero_low)).unwrap();
        signal.set(4, &(one_high, one_low)).unwrap();

        // 发送信号（发送序列）
        tx.start(signal).unwrap();

        FreeRtos::delay_ms(1000);
    });

    loop {
        FreeRtos::delay_ms(3000);
    }
}
