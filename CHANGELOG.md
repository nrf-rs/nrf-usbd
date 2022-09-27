# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.2.0 - 2022-09-27

- Update to `critical-section` 1.0 ([#9]).
- Change `Usbd::new` to return `Self` ([#12]).

[#9]: https://github.com/nrf-rs/nrf-usbd/pull/9
[#12]: https://github.com/nrf-rs/nrf-usbd/pull/12

## 0.1.1 - 2022-01-03

- Use `critical-section` for critical sections instead of `cortex_m::interrupt::free`. This allows
customizing the critical section implementation, needed for using with the nRF SoftDevices.

## 0.1.0 - 2021-07-07

- Initial release
