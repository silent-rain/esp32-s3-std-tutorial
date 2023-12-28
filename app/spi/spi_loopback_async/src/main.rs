use esp_idf_svc::{
    hal::{
        peripherals::Peripherals,
        prelude::FromValueType,
        spi::{Operation, SpiConfig, SpiDeviceDriver, SpiDriver, SpiDriverConfig, SPI2},
        task::block_on,
    },
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

    let spi = peripherals.spi2;

    let sclk = peripherals.pins.gpio15;
    let serial_in = peripherals.pins.gpio16; // SDI
    let serial_out = peripherals.pins.gpio17; // SDO
    let cs_1 = peripherals.pins.gpio18;
    let cs_2 = peripherals.pins.gpio19;

    println!("Starting SPI loopback test");

    let driver = SpiDriver::new::<SPI2>(
        spi,
        sclk,
        serial_out,
        Some(serial_in),
        &SpiDriverConfig::new(),
    )?;

    let config_1 = SpiConfig::new().baudrate(26.MHz().into());
    let mut device_1 = SpiDeviceDriver::new(&driver, Some(cs_1), &config_1)?;

    let config_2 = SpiConfig::new().baudrate(13.MHz().into());
    let mut device_2 = SpiDeviceDriver::new(&driver, Some(cs_2), &config_2)?;

    let write = [0xde, 0xad, 0xbe, 0xef];
    let write_buf = [0xde, 0xad, 0xbe, 0xef];

    let mut read = [0u8; 4];
    let mut write_in_place_buf = [0xde, 0xad, 0xbe, 0xef];
    let mut read_buf = [0; 8];

    block_on(async {
        loop {
            device_1.transfer_async(&mut read, &write).await?;
            println!("Device 1: Wrote {write:x?}, read {read:x?}");

            println!("Device 2: To write {write_in_place_buf:x?} ... ");
            // cascade multiple operations with different buffer length into one transaction
            device_2
                .transaction_async(&mut [
                    Operation::Write(&write_buf),
                    Operation::TransferInPlace(&mut write_in_place_buf),
                    Operation::Read(&mut read_buf),
                ])
                .await?;
            println!("... read {write_in_place_buf:x?}");
        }
    })
}
