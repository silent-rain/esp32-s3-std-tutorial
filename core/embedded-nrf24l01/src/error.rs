use core::fmt::Debug;

/// Wraps an SPI error
///
/// TODO: eliminate this?
#[derive(Debug)]
pub enum Error<SPIE: Debug> {
    /// Wrap an SPI error
    SpiError(SPIE),
}

impl<SPIE: Debug> From<SPIE> for Error<SPIE> {
    fn from(e: SPIE) -> Self {
        Error::SpiError(e)
    }
}
