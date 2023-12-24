//! Setup parameters for SPI
use esp_idf_hal::spi::config::{Mode, Phase, Polarity};

/// SPI setup parameters
pub fn spi_mode() -> Mode {
    Mode {
        polarity: Polarity::IdleLow,
        phase: Phase::CaptureOnFirstTransition,
    }
}

/// Recommended SPI clock speed
///
/// Use as rough guidance.
pub fn clock_mhz() -> u32 {
    8
}
