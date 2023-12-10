use crate::config::Configuration;
use crate::device::Device;
use crate::rx::RxMode;
use crate::tx::TxMode;
use core::fmt;

/// Represents **Standby-I** mode
///
/// This represents the state the device is in inbetween TX or RX
/// mode.
pub struct StandbyMode<D: Device> {
    device: D,
}

impl<D: Device> fmt::Debug for StandbyMode<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StandbyMode")
    }
}

impl<D: Device> StandbyMode<D> {
    /// Constructor
    ///
    /// Puts the `device` into standy mode
    pub fn power_up(mut device: D) -> Result<Self, (D, D::Error)> {
        match device.update_config(|config| config.set_pwr_up(true)) {
            Ok(()) => Ok(StandbyMode { device }),
            Err(e) => Err((device, e)),
        }
    }

    /// Should be a no-op
    pub fn power_down(mut self) -> Result<D, (Self, D::Error)> {
        match self.device.update_config(|config| config.set_pwr_up(false)) {
            Ok(()) => Ok(self.device),
            Err(e) => Err((self, e)),
        }
    }

    pub(crate) fn from_rx_tx(mut device: D) -> Self {
        device.ce_disable();
        StandbyMode { device }
    }

    /// Go into RX mode
    pub fn rx(self) -> Result<RxMode<D>, (D, D::Error)> {
        let mut device = self.device;

        match device.update_config(|config| config.set_prim_rx(true)) {
            Ok(()) => {
                device.ce_enable();
                Ok(RxMode::new(device))
            }
            Err(e) => Err((device, e)),
        }
    }

    /// Go into TX mode
    pub fn tx(self) -> Result<TxMode<D>, (D, D::Error)> {
        let mut device = self.device;

        match device.update_config(|config| config.set_prim_rx(false)) {
            Ok(()) => {
                // No need to device.ce_enable(); yet
                Ok(TxMode::new(device))
            }
            Err(e) => Err((device, e)),
        }
    }
}

impl<D: Device> Configuration for StandbyMode<D> {
    type Inner = D;
    fn device(&mut self) -> &mut Self::Inner {
        &mut self.device
    }
}
