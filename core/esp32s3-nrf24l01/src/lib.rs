// Copyright 2018, Astro <astro@spaceboyz.net>
//
// Licensed under the Apache License, Version 2.0 <LICENSE>. This file
// may not be copied, modified, or distributed except according to
// those terms.

//! nRF24L01+ driver for use with [embedded-hal](https://crates.io/crates/embedded-hal)

#![warn(missing_docs, unused)]
#![no_std]
#[macro_use]
extern crate bitfield;

use core::fmt;
use core::fmt::Debug;

mod config;
use esp_idf_hal::{
    gpio::{AnyIOPin, InputPin, Output, OutputPin, PinDriver},
    peripheral::Peripheral,
    prelude::FromValueType,
    spi::{self, SpiDeviceDriver, SpiDriver, SpiDriverConfig, SPI2},
};

pub use crate::config::{Configuration, CrcMode, DataRate};
pub mod setup;

mod registers;
use crate::registers::{Config, Register, SetupAw, Status};
mod command;
use crate::command::{Command, ReadRegister, WriteRegister};
mod payload;
pub use crate::payload::Payload;
mod error;
pub use crate::error::Error;

mod device;
pub use crate::device::Device;
mod standby;
pub use crate::standby::StandbyMode;
mod rx;
pub use crate::rx::RxMode;
mod tx;
pub use crate::tx::TxMode;

/// Number of RX pipes with configurable addresses
pub const PIPES_COUNT: usize = 6;
/// Minimum address length
pub const MIN_ADDR_BYTES: usize = 3;
/// Maximum address length
pub const MAX_ADDR_BYTES: usize = 5;

/// Driver for the nRF24L01+
///
/// Never deal with this directly. Instead, you store one of the following types:
///
/// * [`StandbyMode<D>`](struct.StandbyMode.html)
/// * [`RxMode<D>`](struct.RxMode.html)
/// * [`TxMode<D>`](struct.TxMode.html)
///
/// where `D: `[`Device`](trait.Device.html)
pub struct NRF24L01<'d, E>
where
    E: Debug,
{
    ce: PinDriver<'d, AnyIOPin, Output>,
    csn: PinDriver<'d, AnyIOPin, Output>,
    spi: SpiDeviceDriver<'d, SpiDriver<'d>>,
    config: Config,
    _e: *const E,
}

impl<'d, E> fmt::Debug for NRF24L01<'d, E>
where
    E: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NRF24L01")
    }
}

impl<'d, E> NRF24L01<'d, E>
where
    E: Debug,
    error::Error<E>: core::convert::From<esp_idf_hal::sys::EspError>,
{
    /// Construct a new driver instance.
    pub fn new(
        spi2: impl Peripheral<P = SPI2> + 'd,
        sclk: impl Peripheral<P = impl OutputPin> + 'd,
        miso: Option<impl Peripheral<P = impl InputPin + OutputPin> + 'd>,
        mosi: impl Peripheral<P = impl OutputPin> + 'd,
        cs: Option<impl Peripheral<P = impl OutputPin> + 'd>,
        ce: AnyIOPin,
        csn: AnyIOPin,
    ) -> Result<StandbyMode<Self>, Error<E>> {
        let driver = SpiDriver::new::<SPI2>(spi2, sclk, mosi, miso, &SpiDriverConfig::new())?;

        let config = spi::config::Config::new().baudrate(26.MHz().into());
        let spi_device = SpiDeviceDriver::new(driver, cs, &config)?;

        let mut ce_device = PinDriver::output(ce)?;
        let mut csn_device = PinDriver::output(csn)?;

        ce_device.set_low()?;
        csn_device.set_high()?;

        // Reset value
        let mut config = Config(0b0000_1000);
        config.set_mask_rx_dr(false);
        config.set_mask_tx_ds(false);
        config.set_mask_max_rt(false);
        let mut device = NRF24L01 {
            ce: ce_device,
            csn: csn_device,
            spi: spi_device,
            config,
            _e: core::ptr::null(),
        };
        assert!(device.is_connected()?);

        // TODO: activate features?

        Ok(StandbyMode::power_up(device).unwrap())
    }

    /// Reads and validates content of the `SETUP_AW` register.
    pub fn is_connected(&mut self) -> Result<bool, Error<E>> {
        let (_, setup_aw) = self.read_register::<SetupAw>()?;
        let valid = setup_aw.aw() >= 3 && setup_aw.aw() <= 5;
        Ok(valid)
    }
}

impl<'d, E> Device for NRF24L01<'d, E>
where
    E: Debug,
{
    type Error = Error<E>;

    fn ce_enable(&mut self) {
        self.ce.set_high().unwrap();
    }

    fn ce_disable(&mut self) {
        self.ce.set_low().unwrap();
    }

    fn send_command<C: Command>(
        &mut self,
        command: &C,
    ) -> Result<(Status, C::Response), Self::Error> {
        // Allocate storage
        let mut read_storage = [0; 33];
        let mut write_storage = [0; 33];
        let len = command.len();
        let read = &mut read_storage[0..len];
        let write = &mut write_storage[0..len];
        // Serialize the command
        command.encode(write);

        // SPI transaction
        self.csn.set_low().unwrap();
        let transfer_result = self.spi.transfer(read, write).map(|_| {});
        self.csn.set_high().unwrap();
        // Propagate Err only after csn.set_high():
        transfer_result.unwrap();

        // Parse response
        let status = Status(read[0]);
        let response = C::decode_response(read);

        Ok((status, response))
    }

    fn write_register<R: Register>(&mut self, register: R) -> Result<Status, Self::Error> {
        let (status, ()) = self.send_command(&WriteRegister::new(register))?;
        Ok(status)
    }

    fn read_register<R: Register>(&mut self) -> Result<(Status, R), Self::Error> {
        self.send_command(&ReadRegister::new())
    }

    fn update_config<F, R>(&mut self, f: F) -> Result<R, Self::Error>
    where
        F: FnOnce(&mut Config) -> R,
    {
        // Mutate
        let old_config = self.config.clone();
        let result = f(&mut self.config);

        if self.config != old_config {
            let config = self.config.clone();
            self.write_register(config)?;
        }
        Ok(result)
    }
}
