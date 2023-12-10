# embedded-nrf24l01

## Features

* Designed for use with the [embedded-hal] crate
* Safe and declarative register definitions
* Chip operation modes lifted to the type-level
* Lets you go straight into RX/TX with the default config

### Still missing

* Auto-ack support

## Reference datasheets

* [nRF24L01+](https://www.sparkfun.com/datasheets/Components/SMD/nRF24L01Pluss_Preliminary_Product_Specification_v1_0.pdf)

## Usage

### Parameters

Get the `*-hal` crate for your micro-controller unit. Figure out how
to get to the peripherals implementing these [embedded-hal] traits:

* `embedded_hal::blocking::spi::Transfer` for the SPI peripheral

  We provide a `mod setup` with a few constants for SPI.
 
* `embedded_hal::digital::OutputPin` for the **CE** pin

* `embedded_hal::digital::OutputPin` for the **CSN** pin

  (Although that one belongs to the SPI, we found it much more
  reliable to implement in software.)

### Constructor

```rust
let mut nrf24 = NRF24L01::new(ce, csn, spi).unwrap();
```

This will provide an instance of `Standby`. You can use `.rx()` or
`.tx()` to transfer into a `RXMode` and `TXMode` instances. They
implement `.standby()` methods to get back to `Standby` and then
switch to the other mode.

### `RXMode`

Use `rx.can_read()` to poll (returning the pipe number), then
`rx.read()` to receive payload.

### `TXMode`

Use `tx.send()` to enqueue a packet.

Use `tx.can_send()` to prevent sending on a full queue, and
`tx.wait_empty()` to flush.


[embedded-hal]: https://crates.io/crates/embedded-hal
