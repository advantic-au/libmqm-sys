# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.10.2](https://github.com/advantic-au/libmqm-sys/compare/libmqm-sys-v0.10.1...libmqm-sys-v0.10.2) - 2026-02-09

### Other

- IBM MQ 9.4.4 and 9.4.5
- Bump dependencies
- Addressed clippy warnings / errors

## [0.10.1](https://github.com/advantic-au/libmqm-sys/compare/libmqm-sys-v0.10.0...libmqm-sys-v0.10.1) - 2025-07-06

### Fixed

- Feature gate MQ 9.4.3 constants
- Removed feature gates for MQDCC

## [0.10.0](https://github.com/advantic-au/libmqm-sys/compare/libmqm-sys-v0.9.0...libmqm-sys-v0.10.0) - 2025-07-05

### Fixed

- MQCBC type ([#116](https://github.com/advantic-au/libmqm-sys/pull/116))

### Other

- Bindings Refactor ([#114](https://github.com/advantic-au/libmqm-sys/pull/114))

## [0.9.0](https://github.com/advantic-au/libmqm-sys/compare/libmqm-sys-v0.8.0...libmqm-sys-v0.9.0) - 2025-06-27

### Added

- bitflags enhancements ([#80](https://github.com/advantic-au/libmqm-sys/pull/80))

### Other

- Release version alignment for libmqm-default and libmqm-sys
- MQ 9.4.3.0 ([#103](https://github.com/advantic-au/libmqm-sys/pull/103))
- Rustify doc comments ([#109](https://github.com/advantic-au/libmqm-sys/pull/109))
- Pregen build improvements ([#97](https://github.com/advantic-au/libmqm-sys/pull/97))
- constant lookup feature for libmqm-sys ([#94](https://github.com/advantic-au/libmqm-sys/pull/94))
- s390x and powerpc64 support ([#90](https://github.com/advantic-au/libmqm-sys/pull/90))
- Linux aarch64 ([#83](https://github.com/advantic-au/libmqm-sys/pull/83))
- Rust 1.82 and Bindgen upgrade ([#81](https://github.com/advantic-au/libmqm-sys/pull/81))

## [0.8.0](https://github.com/advantic-au/libmqm-sys/compare/libmqm-sys-v0.7.0...libmqm-sys-v0.8.0) - 2025-04-24

### Other

- MQ Exit trait and feature refinement ([#77](https://github.com/advantic-au/libmqm-sys/pull/77))
- Rework of some constant generation ([#74](https://github.com/advantic-au/libmqm-sys/pull/74))
- Individual crate versions ([#72](https://github.com/advantic-au/libmqm-sys/pull/72))
- New types ([#69](https://github.com/advantic-au/libmqm-sys/pull/69))
- constants docrs and feature sorting ([#76](https://github.com/advantic-au/libmqm-sys/pull/76))
- libmqm-constants documentation and release configuration ([#71](https://github.com/advantic-au/libmqm-sys/pull/71))

## [0.7.0](https://github.com/advantic-au/libmqm-sys/compare/v0.6.0...v0.7.0) - 2025-03-26

### Other

- IBM MQ client 9.4.2 ([#60](https://github.com/advantic-au/libmqm-sys/pull/60))

- MQC 9.4.1.1

## [0.6.0](https://github.com/advantic-au/libmqm-sys/compare/v0.5.0...v0.6.0) - 2024-12-03

### Other

- Added libmqm_default package to publish MQ structure constants
- Added missing MQI and MQAI functions
- Added some documentation and doctests
- Exposes compiled version constants

## [0.5.0](https://github.com/advantic-au/libmqm-sys/compare/v0.4.0...v0.5.0) - 2024-10-28

### Other

- MQ client 9.2 - 9.4 support
- MQ client version feature flags
- Minor updates on variable naming conventions
