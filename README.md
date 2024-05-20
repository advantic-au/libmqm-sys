libmqm-sys
==========

Native bindings to the IBM® MQ Interface (MQI) and MQ Administration Interface (MQAI)

You can use `libmqm-sys` to:

- Connect to an IBM MQ server to send and receive MQ messages through the MQI functions
- Administer IBM MQ server through the MQAI functions

Functions provided in this crate are the raw `unsafe` functions exposed from the IBM provided library.
Crate users should build safe rust API wrappers over these functions.

Compile time dynamic linking and run-time dynamic linking is supported.

Usage
-----

1. Download and install the redistributable client from IBM:
  [https://ibm.biz/mq93redistclients]

2. Install the client in `/opt/mqm` or another location.

3. Set the MQ_HOME environment variable to the installed location.

4. Add the following to your `Cargo.toml`

    ```toml
    [dependencies]
    libmqm-sys = "0.1.0"
    ```

5. Use the crate in your source code:

```rust
use libmqm-sys as mqsys;
```

Example
-------

Connect to the default queue manager using the MQSERVER environment variable.

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

Feature flags
-------------

| Feature        | Description |
|----------------|-------------|
| link (default) | Support linking the MQ library at compile-time |
| dlopen2        | Support loading the MQ library at run-time using dlopen2 |
| mqai           | Expose the MQAI functions |
| pcf            | Generate the PCF structures |
| exits          | Generate the exit structures |

Status
------

The following needs further work:

- Testing and support on MacOS.
- Documentation.
- Test and support older versions of MQI.
- Add complex examples.

Support
-------

There are no guarantees of compatibility with any future versions of the crate; the API is subject to change based on feedback and enhancements.
Relentless refactoring may occur before a stable release is finalised.

This crate is provided as-is with no guarantees of support or updates.

This crate is not approved, endorsed, acknowledged, or supported by IBM. You cannot use IBM formal support channels (Cases/PMRs) for assistance on the use of this crate.

Contributions
-------------

All feedback, suggestions, contributions and enhancements are welcome.

License
-------

Licensed under

- Apache License, Version 2.0
    ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
