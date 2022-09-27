[![crates.io](https://img.shields.io/crates/d/nrf-usbd.svg)](https://crates.io/crates/nrf-usbd)
[![Documentation](https://img.shields.io/docsrs/nrf-usbd)](https://docs.rs/nrf-usbd)
![Build Status](https://github.com/nrf-rs/nrf-usbd/workflows/CI/badge.svg)

# `nrf-usbd`

[`usb-device`](https://github.com/rust-embedded-community/usb-device) implementation for Nordic
Semiconductor nRF microcontrollers.

## Supported microcontrollers

* `nrf52840`
* `nrf52833`
* `nrf52820`
* `nrf5340`, maybe?

## Usage

This driver is relatively low-level, and is intended for use through a HAL library.
Such HAL library should implement `UsbPeripheral` for the corresponding USB peripheral object.
This trait declares all the peripheral properties that may vary from one device family to the other.

## Examples

See the [`nrf-hal`](https://github.com/nrf-rs/nrf-hal) for the reference HAL implementation.

See the [`example`](./example) directory for an example on how to use it standalone without a HAL.
This is discouraged, the recommended usage is through `nrf-hal`.
