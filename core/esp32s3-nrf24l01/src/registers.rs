#![allow(unused)]

use crate::{MAX_ADDR_BYTES, MIN_ADDR_BYTES, PIPES_COUNT};

pub trait Register {
    /// Address in the register map
    fn addr() -> u8;

    fn read_len() -> usize;
    fn write_len(&self) -> usize {
        Self::read_len()
    }

    fn encode(&self, data: &mut [u8]);
    fn decode(data: &[u8]) -> Self;
}

macro_rules! def_simple {
    ($name: ident) => {
        pub struct $name(pub u8);

        impl $name {
            pub fn new(data: &[u8]) -> Self {
                assert_eq!(data.len(), 1);

                $name(data[0])
            }
        }
    };
}

/// Common for all registers with 1 bytes of data
macro_rules! impl_register {
    ($name: ident, $addr: expr) => {
        impl Register for $name {
            fn addr() -> u8 {
                $addr
            }

            fn read_len() -> usize {
                1
            }

            fn encode(&self, buf: &mut [u8]) {
                buf[0] = self.0;
            }

            fn decode(buf: &[u8]) -> Self {
                $name(buf[0])
            }
        }

        impl Clone for $name {
            fn clone(&self) -> Self {
                $name(self.0)
            }
        }

        impl PartialEq for $name {
            fn eq(&self, rhs: &Self) -> bool {
                self.0 == rhs.0
            }
        }
    };
}

macro_rules! def_address_register {
    ($name: ident, $addr: expr) => {
        pub struct $name {
            addr: [u8; MAX_ADDR_BYTES],
            len: u8,
        }

        impl $name {
            pub fn new(buf: &[u8]) -> Self {
                Self::decode(buf)
            }
        }

        impl Register for $name {
            fn addr() -> u8 {
                $addr
            }

            fn read_len() -> usize {
                MAX_ADDR_BYTES
            }

            fn write_len(&self) -> usize {
                self.len.into()
            }

            fn encode(&self, buf: &mut [u8]) {
                let len = self.len.into();
                buf.copy_from_slice(&self.addr[0..len]);
            }

            fn decode(buf: &[u8]) -> Self {
                let len = buf.len();
                assert!(len >= MIN_ADDR_BYTES);
                assert!(len <= MAX_ADDR_BYTES);

                let mut addr = [0; MAX_ADDR_BYTES];
                addr[0..len].copy_from_slice(buf);
                $name {
                    addr,
                    len: len as u8,
                }
            }
        }
    };
}

macro_rules! def_pipes_accessors {
    ($name: ident, $default: expr, $getter: ident, $setter: ident) => {
        impl $name {
            #[inline]
            pub fn $getter(&self, pipe_no: usize) -> bool {
                let mask = 1 << pipe_no;
                self.0 & mask == mask
            }

            #[inline]
            pub fn $setter(&mut self, pipe_no: usize, enable: bool) {
                let mask = 1 << pipe_no;
                if enable {
                    self.0 |= mask;
                } else {
                    self.0 &= !mask;
                }
            }

            pub fn from_bools(bools: &[bool; PIPES_COUNT]) -> Self {
                let mut register = $name($default);
                for (i, b) in bools.iter().enumerate() {
                    register.$setter(i, *b);
                }
                register
            }

            pub fn to_bools(&self) -> [bool; PIPES_COUNT] {
                let mut bools = [true; PIPES_COUNT];
                for (i, b) in bools.iter_mut().enumerate() {
                    *b = self.$getter(i);
                }
                bools
            }
        }
    };
}

bitfield! {
    pub struct Config(u8);
    impl Debug;

    /// Mask interrupt
    pub mask_rx_dr, set_mask_rx_dr: 6;
    /// Mask interrupt
    pub mask_tx_ds, set_mask_tx_ds: 5;
    /// Mask interrupt
    pub mask_max_rt, set_mask_max_rt: 4;
    /// Enable CRC
    pub en_crc, set_en_crc: 3;
    /// CRC encoding scheme
    /// * `0`: 1 byte
    /// * `1`: 2 bytes
    pub crco, set_crco: 2;
    /// Power up
    pub pwr_up, set_pwr_up: 1;
    /// RX/TX control
    /// * `1`: PRX
    /// * `0`: PTX
    pub prim_rx, set_prim_rx: 0;
}
impl_register!(Config, 0x00);

/// Enable Auto Acknowledgment
#[derive(Debug)]
pub struct EnAa(pub u8);
impl_register!(EnAa, 0x01);
def_pipes_accessors!(EnAa, 0b0011_1111, enaa_p, set_enaa_p);

/// Enabled RX Addresses
#[derive(Debug)]
pub struct EnRxaddr(u8);
impl_register!(EnRxaddr, 0x02);
def_pipes_accessors!(EnRxaddr, 0, erx_p, set_erx_p);

bitfield! {
    pub struct SetupAw(u8);
    impl Debug;

    /// RX/TX address field width:
    /// * `0b01`: 3 bytes
    /// * `0b10`: 4 bytes
    /// * `0b11`: 5 bytes
    pub u8, aw, set_aw: 1, 0;
}
impl_register!(SetupAw, 0x03);

bitfield! {
    /// Setup of Automatic Retransmission
    pub struct SetupRetr(u8);
    impl Debug;

    /// Auto Retransmit Delay, where the actualy delay is `250 + (250 * ard) µS`
    pub u8, ard, set_ard: 7, 4;
    /// Auto Retransmit Count
    pub u8, arc, set_arc: 3, 0;
}
impl_register!(SetupRetr, 0x04);

bitfield! {
    /// RF Channel
    pub struct RfCh(u8);
    impl Debug;

    /// Frequency, that is `2400 + rf_ch` Mhz
    pub u8, rf_ch, set_rf_ch: 6, 0;
}
impl_register!(RfCh, 0x05);

bitfield! {
    /// RF Setup
    pub struct RfSetup(u8);
    impl Debug;

    /// Set for 250 kbps
    pub rf_dr_low, set_rf_dr_low: 5;
    /// Set for 2 Mbps
    pub rf_dr_high, set_rf_dr_high: 3;
    /// RF output power in TX mode
    /// * `00`: -18 dBm
    /// * `01`: -12 dBm
    /// * `10`: -6 dBm
    /// * `11`: 0 dBm
    pub u8, rf_pwr, set_rf_pwr: 2, 1;
}
impl_register!(RfSetup, 0x06);

bitfield! {
    /// Status register, always received on MISO while command is sent
    /// on MOSI.
    pub struct Status(u8);
    impl Debug;

    /// Data ready RX FIFO interrupt. Write `true` to clear.
    pub rx_dr, set_rx_dr: 6;
    /// Data sent TX FIFO interrupt. Write `true` to clear.
    pub tx_ds, set_tx_ds: 5;
    /// Maximum number of TX retransmits interrupt. Write `true` to clear.
    pub max_rt, set_max_rt: 4;
    /// Data pipe number for reading from RX FIFO
    pub u8, rx_p_no, _: 3, 1;
    /// TX FIFO full flag
    pub tx_full, _: 0;
}
impl_register!(Status, 0x07);

bitfield! {
    pub struct ObserveTx(u8);
    impl Debug;

    pub u8, plos_cnt, _: 7, 4;
    pub u8, arc_cnt, _: 3, 0;
}
impl_register!(ObserveTx, 0x08);

def_simple!(CD);
impl_register!(CD, 0x09);

def_address_register!(RxAddrP0, 0x0A);
def_address_register!(RxAddrP1, 0x0B);
def_simple!(RxAddrP2);
impl_register!(RxAddrP2, 0x0C);
def_simple!(RxAddrP3);
impl_register!(RxAddrP3, 0x0D);
def_simple!(RxAddrP4);
impl_register!(RxAddrP4, 0x0E);
def_simple!(RxAddrP5);
impl_register!(RxAddrP5, 0x0F);

def_address_register!(TxAddr, 0x10);

macro_rules! def_rx_pw {
    ($name: ident, $addr: expr) => {
        bitfield! {
            /// Static payload length for RX
            pub struct $name(u8);
            impl Debug;

            /// Number of bytes in RX payload in data pipe (max: 32)
            pub u8, get, set: 5, 0;
        }
        impl_register!($name, $addr);
    };
}

def_rx_pw!(RxPwP0, 0x11);
def_rx_pw!(RxPwP1, 0x12);
def_rx_pw!(RxPwP2, 0x13);
def_rx_pw!(RxPwP3, 0x14);
def_rx_pw!(RxPwP4, 0x15);
def_rx_pw!(RxPwP5, 0x16);

bitfield! {
    /// Status register, always received on MISO while command is sent
    /// on MOSI.
    pub struct FifoStatus(u8);
    impl Debug;

    pub tx_reuse, _: 6;
    /// TX FIFO full flag
    pub tx_full, _: 5;
    /// TX FIFO empty flag
    pub tx_empty, _: 4;
    /// RX FIFO full flag
    pub rx_full, _: 1;
    /// RX FIFO empty flag
    pub rx_empty, _: 0;
}
impl_register!(FifoStatus, 0x17);

/// Enable Dynamic Payload length
pub struct Dynpd(pub u8);
impl_register!(Dynpd, 0x1C);
def_pipes_accessors!(Dynpd, 0, dpl_p, set_dpl_p);

bitfield! {
    /// Enable features
    pub struct Feature(u8);
    impl Debug;

    /// Enables Dynamic Payload Length
    pub en_dpl, set_en_dpl: 2;
    /// Enables Payload with ACK
    pub en_ack_pay, set_en_ack_pay: 1;
    /// Enables the W_TX_PAYLOAD_NOACK command
    pub en_dyn_ack, set_en_dyn_ack: 0;
}
impl_register!(Feature, 0x1D);
