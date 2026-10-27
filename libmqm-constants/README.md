libmqm-constants - IBM MQ constants for Rust
============================================

[![Latest version](https://img.shields.io/crates/v/libmqm-constants.svg)](https://crates.io/crates/libmqm-constants)
[![Documentation](https://docs.rs/libmqm-constants/badge.svg)](https://docs.rs/libmqm-constants)
![License](https://img.shields.io/crates/l/libmqm-constants.svg)

Programming constants for the IBM® MQ Interface (MQI) and MQ Administration Interface (MQAI) libraries.

`libmqm-constants` provides typed Rust constants for IBM MQ, along with lookup between
constant values and their MQ names. Use it alongside [`libmqm-sys`](../README.md) for the
raw bindings, or [`mqi`](https://github.com/advantic-au/mqi) for a safe application API.

Key Features
------------

- **MQI and MQAI**: Constants covering the MQI and MQAI APIs.
- **New types**: Typed constants that make MQ values easier to distinguish and use.
- **Name lookup**: Convert typed constants to their MQ names.

Usage
-----

```rust
use libmqm_constants::{constants, lookup::HasMqNames as _};

let reason = constants::MQRC_NONE;
assert_eq!(reason.to_string(), "MQRC_NONE");
assert_eq!(reason.mq_primary_name(), Some("MQRC_NONE"));
```

The constants are generated from the MQ client headers and committed with the crate.
Normal builds therefore do not need the MQ headers. Enable the `mqai`, `pcf`, or `exits`
features to include constants for those APIs.
