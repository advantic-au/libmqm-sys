libmqm-sys
==========

[![Latest version](https://img.shields.io/crates/v/libmqm-sys.svg)](https://crates.io/crates/libmqm-sys)
[![Documentation](https://docs.rs/libmqm-sys/badge.svg)](https://docs.rs/libmqm-sys)
![License](https://img.shields.io/crates/l/libmqm-sys.svg)

Native bindings to the IBM® MQ Interface (MQI) and MQ Administration Interface (MQAI)

You can use `libmqm-sys` to:

- Connect to an IBM MQ server to send and receive MQ messages through the MQI functions
- Administer IBM MQ server through the MQAI functions

Functions provided in this crate are the raw `unsafe` functions exposed from the
IBM provided library. Developers should build safe rust API wrappers over these functions.
Developers who do not want to use the unsafe API should use the
[mqi](https::/github.com/advantic-au/mqi) crate for a *safe* API over the MQI.

Compile time dynamic linking and run-time dynamic linking is supported.

Developers must download the [MQI library](https://ibm.biz/mq94redistclients) directly from IBM.
Refer to the [Usage] instructions.

Usage
-----

1. Download and install the redistributable client from IBM:
  <https://ibm.biz/mq94redistclients>

2. Install the client in `/opt/mqm` or another location.

3. Set the MQ_HOME environment variable to the installed location.

    ```bash
    MQ_HOME=/opt/mqm
    ```

4. Add the following to your `Cargo.toml`

    ```toml
    [dependencies]
    libmqm-sys = "0.4.0"
    ```

5. Use the crate in your source code:

    ```rust
    use libmqm-sys as mqsys;
    ```

Example
-------

```rust
#[cfg(test)]
mod test {
    use std::ptr::addr_of_mut;
    use libmqm_sys::lib;

    #[test]
    fn connect() {
        unsafe {
            let mut hconn = lib::MQHC_DEF_HCONN;
            let mut comp_code = lib::MQCC_UNKNOWN;
            let mut reason = lib::MQRC_NONE;
            let mut qmgr: [i8; 48] = [b' '; 48]; // All spaces
            lib::MQCONN(
                addr_of_mut!(qmgr).cast(),
                addr_of_mut!(hconn),
                addr_of_mut!(comp_code),
                addr_of_mut!(reason),
            );
            assert_eq!(reason, lib::MQRC_NONE, "MQRC");
            assert_eq!(comp_code, lib::MQCC_OK, "MQCC");
            lib::MQDISC(addr_of_mut!(hconn), addr_of_mut!(comp_code), addr_of_mut!(reason));
        };
    }
}
```

For further examples of using the API, refer to the [MQI crate usage](https://github.com/advantic-au/mqi/blob/develop/src/core/mqi_verbs.rs).

Feature flags
-------------

| Feature           | Description |
|-------------------|-------------|
| link (default)    | Support linking the MQ library at compile-time |
| bindgen (default) | Generate the bindings from MQI library |
| dlopen2           | Support loading the MQ library at run-time using [`dlopen2`](https://crates.io/crates/dlopen2) |
| mqai              | Expose the MQAI functions |
| pcf               | Generate the PCF structures |
| exits             | Generate the exit structures |

Status
------

The following needs further work:

- Documentation.
- Test and support older versions of MQI.

Contributions
-------------

All feedback, suggestions, contributions and enhancements are welcome.

Support
-------

There are no guarantees of compatibility with any future versions of the crate; the API
is subject to change based on feedback and enhancements. Relentless refactoring may occur
before a stable crate is released.

This crate is provided as-is with no guarantees of support or updates.

**This crate is not approved, endorsed, acknowledged, or supported by IBM. You cannot use
IBM formal support channels (Cases/PMRs) for assistance on the use of this crate.**

License
-------

Licensed under

- Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
