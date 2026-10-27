libmqm-default - IBM MQ structure defaults
==========================================

[![Latest version](https://img.shields.io/crates/v/libmqm-default.svg)](https://crates.io/crates/libmqm-default)
[![Documentation](https://docs.rs/libmqm-default/badge.svg)](https://docs.rs/libmqm-default)
![License](https://img.shields.io/crates/l/libmqm-default.svg)

`libmqm-default` provides ready-to-use default initializers for IBM MQ C structures.
Use it with [`libmqm-sys`](../README.md) when calling the raw MQ API directly.
For typical application development, the safe [`mqi`](https://github.com/advantic-au/mqi)
crate is a higher-level alternative.

Usage
-----

```rust
let mut message_descriptor = libmqm_default::MQMD2_DEFAULT;
assert_eq!(message_descriptor.Version, libmqm_sys::MQMD_VERSION_2);
```

The defaults are generated from IBM MQ headers and committed with the crate, so normal
builds do not need the headers. Enable `pcf` or `exits` to include defaults for those APIs.

License
-------

Licensed under Apache License, Version 2.0
([LICENSE-APACHE](../LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>).
