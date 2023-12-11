use core::ops::Deref;

/// Represents a received packet. Stores 32 bytes and the actual length.
///
/// Use [`as_ref()`](#method.as_ref) or [`Deref`](#impl-Deref) to
/// obtain a slice of the content.
pub struct Payload {
    data: [u8; 32],
    len: usize,
}

impl Payload {
    /// Copy a slice
    pub fn new(source: &[u8]) -> Self {
        let mut data = [0; 32];
        let len = source.len().min(data.len());
        data[0..len].copy_from_slice(&source[0..len]);
        Payload { data, len }
    }

    /// Read length
    pub fn len(&self) -> usize {
        self.len
    }
}

impl AsRef<[u8]> for Payload {
    fn as_ref(&self) -> &[u8] {
        &self.data[0..self.len]
    }
}

impl Deref for Payload {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.as_ref()
    }
}
