use w25q64::reg::W25Q64;

use esp_idf_svc::{
    hal::{delay::FreeRtos, peripherals::Peripherals},
    log::EspLogger,
    sys::link_patches,
};

fn main() -> anyhow::Result<()> {
    link_patches();
    // Bind the log crate to the ESP Logging facilities
    EspLogger::initialize_default();
    // 设置日志级别
    log::set_max_level(log::LevelFilter::Info);

    // Get the peripherals
    let peripherals = Peripherals::take()?;

    let cs = peripherals.pins.gpio4;
    let sck = peripherals.pins.gpio5;
    let mosi = peripherals.pins.gpio6;
    let miso = peripherals.pins.gpio7;
    let mut w25q = W25Q64::new(cs.into(), sck.into(), mosi.into(), miso.into())?;

    let (mid, did) = w25q.read_id()?;
    log::info!("mid: {:02X}, did: {:02X}", mid, did);

    // 读取芯片的JEDEC设备ID
    let (manufacturer_id, memory_type, capacity) = w25q.read_jedec_device_id()?;
    log::info!(
        "manufacturer_id: {:02X}, memory_type: {:02X}, capacity: {:02X}",
        manufacturer_id,
        memory_type,
        capacity
    );

    // 读取芯片的制造商和设备ID
    let (manufacturer_id, device_id) = w25q.read_manufacturer_device_id()?;
    log::info!(
        "manufacturer_id: {:02X}, device_id: {:02X}",
        manufacturer_id,
        device_id
    );

    let tx_buf: [u8; 4] = [0x01, 0x02, 0x03, 0x04];
    let mut rx_buf: [u8; 4] = [0; 4];

    log::info!("sector_erase");
    w25q.sector_erase(0x000000)?;
    log::info!("page_program");
    w25q.page_program(0x000000, &tx_buf)?;

    w25q.read_data(0x000000, &mut rx_buf)?;
    log::info!("read: {:?}", rx_buf);

    log::info!("loop");
    loop {
        FreeRtos::delay_ms(1000);
    }
}
