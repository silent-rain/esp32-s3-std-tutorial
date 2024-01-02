use std::fmt::Write;

use esp_idf_hal::{
    sys::{EspError, TickType_t},
    uart::UartDriver,
};

/// 发送字节
pub fn send_byte(uart: &mut UartDriver<'_>, word: u8) -> Result<usize, EspError> {
    uart.write(&[word])
}

/// 发送字节数组
pub fn send_bytes(uart: &mut UartDriver<'_>, words: &[u8]) -> Result<usize, EspError> {
    uart.write(words)
}

/// 发送字符串
pub fn send_string(uart: &mut UartDriver<'_>, words: &str) -> Result<(), std::fmt::Error> {
    uart.write_str(words)
}

/// 发送数字
pub fn send_number(uart: &mut UartDriver<'_>, number: u32) -> Result<(), std::fmt::Error> {
    uart.write_str(number.to_string().as_str())
}

/// 接收字节
pub fn recv_byte(uart: &mut UartDriver<'_>, timeout: TickType_t) -> Result<u8, EspError> {
    let mut buf = [0_u8; 1];
    uart.read(&mut buf, timeout)?;
    Ok(buf[0])
}

/// 接收字节数组
/// 结束符: b'\n'
pub fn recv_bytes(
    uart: &mut UartDriver<'_>,
    buffer: &mut [u8],
    timeout: TickType_t,
) -> Result<(), EspError> {
    let mut widx: usize = 0;
    loop {
        let mut buf = [0_u8; 1];
        uart.read(&mut buf, timeout)?;
        let w = buf[0];
        if w == b'\n' {
            break;
        }
        if widx < buffer.len() {
            buffer[widx] = w;
            widx += 1;
        }
    }

    Ok(())
}

/// 接收字符串
pub fn recv_string(uart: &mut UartDriver<'_>, timeout: TickType_t) -> Result<String, EspError> {
    let mut s = String::new();
    loop {
        let mut buf = [0_u8; 1];
        uart.read(&mut buf, timeout)?;
        let w = buf[0];
        if w == b'\n' {
            break;
        }
        s.push(w as char);
    }
    Ok(s)
}
