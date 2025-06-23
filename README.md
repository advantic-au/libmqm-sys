libmqm-sys
==========

[![Latest version](https://img.shields.io/crates/v/libmqm-sys.svg)](https://crates.io/crates/libmqm-sys)
[![Documentation](https://docs.rs/libmqm-sys/badge.svg)](https://docs.rs/libmqm-sys)
![License](https://img.shields.io/crates/l/libmqm-sys.svg)

Bindings to the IBM® MQ Interface (MQI) and MQ Administration Interface (MQAI) libraries.

You can use `libmqm-sys` to:

- Connect to an IBM MQ server to send and receive MQ messages through the MQI functions
- Administer IBM MQ server through the PCF structures and MQAI functions

Functions provided in this crate are the raw `unsafe` functions exposed from the
IBM provided library. Developers should build safe rust API wrappers over these functions.
Developers who do not want to use the unsafe API should use the
[mqi](https://github.com/advantic-au/mqi) crate for a *safe* API over the MQI.

Compile time dynamic linking and run-time dynamic linking is supported.

Developers must download the [MQI library](https://ibm.biz/mq94redistclients) directly from IBM.
Refer to the [Usage](#usage) instructions.

Usage
-----

1. Download and install the client from IBM:
   - Windows/Linux x86-64 redistributable - <https://public.dhe.ibm.com/ibmdl/export/pub/software/websphere/messaging/mqdev/redist/>
   - MacOS (x86-64/ARM64) toolkit - <https://public.dhe.ibm.com/ibmdl/export/pub/software/websphere/messaging/mqdev/mactoolkit/>
   - Linux (x86-64/ARM64/PowerPC64le/S390X) MQ Advanced Developer - <https://public.dhe.ibm.com/ibmdl/export/pub/software/websphere/messaging/mqadv/>

2. Extract and install the MQ client in any location.

3. Set the MQ_HOME environment variable to the installed location.

4. Add the `libmqm-sys` crate to your project:

    ```sh
    cargo add libmqm-sys
    ```

5. Ensure the MQ libraries are in the library search path. On Linux, this can
   be achieved by setting the `LD_LIBRARY_PATH` environment variable.

An easy way of running an MQ server for development and testing is to run the IBM supplied docker container:

```sh
docker run -d --publish 1414:1414 icr.io/ibm-messaging/mq:latest
```

Refer to <https://github.com/ibm-messaging/mq-container/blob/master/docs/usage.md>

Example
-------

```rust
use libmqm_sys::lib;

let mut hconn = lib::MQHC_DEF_HCONN;
let mut comp_code = lib::MQCC_UNKNOWN;
let mut reason = lib::MQRC_NONE;
let mut qmgr: [lib::MQCHAR; 48] = [32; 48]; // All spaces = default qmgr

unsafe {
    lib::MQCONN(
        &qmgr,
        &mut hconn,
        &mut comp_code,
        &mut reason,
    );
    assert_eq!(reason, lib::MQRC_NONE, "MQRC");
    assert_eq!(comp_code, lib::MQCC_OK, "MQCC");
    lib::MQDISC(&mut hconn, &mut comp_code, &mut reason);
};
```

For further examples of using the API, refer to the [MQI crate usage](https://github.com/advantic-au/mqi/blob/develop/src/core/mqi_verbs.rs).

Feature flags
-------------

| Feature           | Default     | Description |
|-------------------|-------------|-------------|
| link              | ✔ | Support linking the MQ library at compile-time |
| bindgen           |   | Generate the bindings from MQI library |
| dlopen2           |   | Support loading the MQ library at run-time using [`dlopen2`](https://crates.io/crates/dlopen2) |
| mqai              |   | Expose the MQAI functions |
| pcf               |   | Generate the PCF structures |
| exits             |   | Generate the exit structures |
| mqc_*             | mqc_9_2_0_0 | Enable features of a specific MQI library version eg `mqc_9_3_1_0` |
| mqc_latest        |   | Enable features of the latest MQI library version |

Status
------

The following needs further work:

- Documentation

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
