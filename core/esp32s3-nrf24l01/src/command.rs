pub use crate::payload::Payload;
use crate::registers::Register;
use core::marker::PhantomData;

pub trait Command {
    fn len(&self) -> usize;
    fn encode(&self, data: &mut [u8]);

    type Response;
    fn decode_response(data: &[u8]) -> Self::Response;
}

pub struct ReadRegister<R: Register> {
    register: PhantomData<R>,
}

impl<R: Register> ReadRegister<R> {
    pub fn new() -> Self {
        ReadRegister {
            register: PhantomData,
        }
    }
}

impl<R: Register> Command for ReadRegister<R> {
    fn len(&self) -> usize {
        1 + R::read_len()
    }

    fn encode(&self, buf: &mut [u8]) {
        buf[0] = R::addr();
    }

    type Response = R;
    fn decode_response(data: &[u8]) -> Self::Response {
        R::decode(&data[1..])
    }
}

pub struct WriteRegister<R: Register> {
    register: R,
}

impl<R: Register> WriteRegister<R> {
    pub fn new(register: R) -> Self {
        WriteRegister { register }
    }
}

impl<R: Register> Command for WriteRegister<R> {
    fn len(&self) -> usize {
        1 + self.register.write_len()
    }

    fn encode(&self, buf: &mut [u8]) {
        buf[0] = 0b10_0000 | R::addr();
        self.register.encode(&mut buf[1..]);
    }

    type Response = ();
    fn decode_response(_: &[u8]) -> Self::Response {}
}

pub struct ReadRxPayload {
    payload_width: usize,
}

impl ReadRxPayload {
    pub fn new(payload_width: usize) -> Self {
        ReadRxPayload { payload_width }
    }
}

impl Command for ReadRxPayload {
    fn len(&self) -> usize {
        1 + self.payload_width
    }

    fn encode(&self, buf: &mut [u8]) {
        buf[0] = 0b0110_0001;
    }

    type Response = Payload;
    fn decode_response(data: &[u8]) -> Self::Response {
        Payload::new(&data[1..])
    }
}

pub struct WriteTxPayload<'a> {
    data: &'a [u8],
}

impl<'a> WriteTxPayload<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        WriteTxPayload { data }
    }
}

impl<'a> Command for WriteTxPayload<'a> {
    fn len(&self) -> usize {
        1 + self.data.len()
    }

    fn encode(&self, buf: &mut [u8]) {
        buf[0] = 0b1010_0000;
        buf[1..].copy_from_slice(self.data);
    }

    type Response = ();
    fn decode_response(_: &[u8]) -> Self::Response {}
}

pub struct ReadRxPayloadWidth;

impl Command for ReadRxPayloadWidth {
    fn len(&self) -> usize {
        2
    }

    fn encode(&self, buf: &mut [u8]) {
        buf[0] = 0b0110_0000;
    }

    type Response = u8;
    fn decode_response(data: &[u8]) -> Self::Response {
        data[1]
    }
}

pub struct FlushRx;

impl Command for FlushRx {
    fn len(&self) -> usize {
        1
    }

    fn encode(&self, buf: &mut [u8]) {
        buf[0] = 0b1110_0010;
    }

    type Response = ();
    fn decode_response(_: &[u8]) -> Self::Response {}
}

pub struct FlushTx;

impl Command for FlushTx {
    fn len(&self) -> usize {
        1
    }

    fn encode(&self, buf: &mut [u8]) {
        buf[0] = 0b1110_0001;
    }

    type Response = ();
    fn decode_response(_: &[u8]) -> Self::Response {}
}

pub struct Nop;

impl Command for Nop {
    fn len(&self) -> usize {
        1
    }

    fn encode(&self, buf: &mut [u8]) {
        buf[0] = 0b1111_1111;
    }

    type Response = ();
    fn decode_response(_: &[u8]) -> Self::Response {}
}
