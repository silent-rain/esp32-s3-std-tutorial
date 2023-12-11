//! Setup parameters for SPI

use embedded_hal::spi;

/// SPI setup parameters
pub fn spi_mode() -> spi::Mode {
    spi::Mode {
        polarity: spi::Polarity::IdleLow,
        phase: spi::Phase::CaptureOnFirstTransition,
    }
}

/// Recommended SPI clock speed
///
/// Use as rough guidance.
pub fn clock_mhz() -> u32 {
    8
}
