use core::fmt;

use anyhow::{Ok, Result};
use esp_idf_hal::prelude::*;
use esp_idf_hal::{
    gpio::{InputPin, Output, OutputPin, PinDriver},
    peripheral::Peripheral,
    spi::{self, SpiDeviceDriver, SpiDriver, SpiDriverConfig, SPI2},
};

/// Driver for the nRF24L01+
pub struct NRF24L01<'d, CE, CSN>
where
    CE: OutputPin,
    CSN: OutputPin,
{
    ce: PinDriver<'d, CE, Output>,
    csn: PinDriver<'d, CSN, Output>,
}

impl<'d, CE, CSN> fmt::Debug for NRF24L01<'d, CE, CSN>
where
    CE: OutputPin,
    CSN: OutputPin,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NRF24L01")
    }
}
/**
 *     /// Reads and validates content of the `SETUP_AW` register.
// pub fn is_connected(&mut self) -> Result<bool, Error<SPIE>> {
//     let (_, setup_aw) = self.read_register::<SetupAw>()?;
//     let valid = setup_aw.aw() >= 3 && setup_aw.aw() <= 5;
//     Ok(valid)
// }
 */
impl<'d, CE, CSN> NRF24L01<'d, CE, CSN>
where
    CE: OutputPin,
    CSN: OutputPin,
{
    /// Construct a new driver instance.
    pub fn new(
        spi2: impl Peripheral<P = SPI2> + 'd,
        sclk: impl Peripheral<P = impl OutputPin> + 'd,
        miso: Option<impl Peripheral<P = impl InputPin + OutputPin> + 'd>,
        mosi: impl Peripheral<P = impl OutputPin> + 'd,
        cs: Option<impl Peripheral<P = impl OutputPin> + 'd>,
        ce: CE,
        csn: CSN,
    ) -> Result<Self> {
        let driver = SpiDriver::new::<SPI2>(spi2, sclk, mosi, miso, &SpiDriverConfig::new())?;

        let config = spi::config::Config::new().baudrate(26.MHz().into());
        let mut spi_device = SpiDeviceDriver::new(&driver, cs, &config)?;

        let mut ce_device = PinDriver::output(ce)?;
        let mut csn_device = PinDriver::output(csn)?;

        ce_device.set_low()?;
        csn_device.set_high()?;

        let nrt24 = NRF24L01 {
            ce: ce_device,
            csn: csn_device,
        };

        Ok(nrt24)
    }
}
