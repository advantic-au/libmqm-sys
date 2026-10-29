libmqm-sys
==========

[![Latest version](https://img.shields.io/crates/v/libmqm-sys.svg)](https://crates.io/crates/libmqm-sys)
[![Documentation](https://docs.rs/libmqm-sys/badge.svg)](https://docs.rs/libmqm-sys)
![License](https://img.shields.io/crates/l/libmqm-sys.svg)

Bindings to the IBM® MQ Interface (MQI) and MQ Administration Interface (MQAI) libraries.

For application development, start with the safe [`mqi` crate](https://github.com/advantic-au/mqi).
Use `libmqm-sys` when you need direct access to IBM MQ's C API or are building a safe wrapper.

You can use `libmqm-sys` to:

- Connect to an IBM MQ server to send and receive MQ messages through the MQI functions
- Administer IBM MQ server through the PCF structures and MQAI functions

Functions provided in this crate are the raw `unsafe` functions exposed from the
IBM-provided library. Their safety requirements come from the MQ C API; use the
[`mqi`](https://github.com/advantic-au/mqi) crate if you prefer a safe Rust API.

Compile-time linking is enabled by default. Runtime loading is also available with the
`dlopen2` feature.

Developers must download the MQ client library or MQ server directly from IBM.
Refer to the [setup](#setup) instructions.

Setup
-----

1. Download and install the IBM MQ client for your platform:
   - Windows/Linux x86-64 redistributable - <https://public.dhe.ibm.com/ibmdl/export/pub/software/websphere/messaging/mqdev/redist/>
   - MacOS (x86-64/ARM64) toolkit - <https://public.dhe.ibm.com/ibmdl/export/pub/software/websphere/messaging/mqdev/mactoolkit/>
   - Linux (x86-64/ARM64/PowerPC64le/S390X) MQ Advanced Developer - <https://public.dhe.ibm.com/ibmdl/export/pub/software/websphere/messaging/mqadv/>

2. Extract and install the MQ client. If it is not installed in the default location
   (`/opt/mqm` on Linux or `C:\Program Files\IBM\MQ` on Windows), set `MQ_HOME` to its
   installation directory so the build can find the MQ library.

3. Add the crate to your project:

    ```sh
    cargo add libmqm-sys
    ```

   The default `link` feature links the MQ library at build time. The MQ shared library
   must also be discoverable when you run your application. For example, on Linux:

    ```sh
    export MQ_HOME="${MQ_HOME:-/opt/mqm}"
    export LD_LIBRARY_PATH="$MQ_HOME/lib64${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
    ```

   On macOS, make the MQ library available through the dynamic loader's search path
   (for example, `DYLD_LIBRARY_PATH`). On Windows, add the MQ `bin` directory to `PATH`.

### Runtime loading

To load the MQ library when the application starts instead of linking it at build time,
disable default features and enable `dlopen2`:

```sh
cargo add libmqm-sys --no-default-features --features dlopen2,mqc_9_3_0_0
```

The MQ shared library must still be installed and discoverable by the operating system
when the application runs. Loading and calling the raw MQ functions is unsafe; see the
[`dlopen2` API documentation](https://docs.rs/libmqm-sys/latest/libmqm_sys/dlopen2/).

### Generating bindings

Normal builds use the bindings committed with the crate and do not need the MQ headers
or bindgen. Generating bindings from headers is a separate, developer workflow: it
requires an MQ installation, `MQ_HOME`, `libclang`, and a C compiler.

An easy way of running an MQ server for development and testing is to run the IBM supplied docker container:

```sh
docker run -d --publish 1414:1414 icr.io/ibm-messaging/mq:latest
```

Refer to <https://github.com/ibm-messaging/mq-container/blob/master/docs/usage.md>

Recommended API
---------------

For typical application code, use [`mqi`](https://github.com/advantic-au/mqi), which
provides a safe, higher-level API. Its README includes a complete
[connect-and-put example](https://github.com/advantic-au/mqi#example).

Raw FFI example
---------------

This example shows the low-level calls only. It assumes the application has handled the
MQ C API's safety requirements, checks each completion code, and reports MQ reason codes.
Successful calls with warnings are reported and the connection is still disconnected.
For production application code, prefer `mqi`.

```rust
use std::error::Error;

use libmqm_sys as mq;

fn main() -> Result<(), Box<dyn Error>> {
    let mut hconn = mq::MQHC_DEF_HCONN;
    let mut comp_code = mq::MQCC_UNKNOWN;
    let mut reason = mq::MQRC_NONE;
    let qmgr: mq::MQCHAR48 = [32; 48]; // All spaces select the default queue manager.

    // SAFETY: The MQ client is installed, the arguments meet the MQI contract, and
    // the returned connection handle is used only after a successful connection.
    unsafe {
        mq::MQCONN(&qmgr, &mut hconn, &mut comp_code, &mut reason);
    }
    match comp_code {
        mq::MQCC_OK => {}
        mq::MQCC_WARNING => eprintln!("MQCONN completed with warning: reason {reason}"),
        _ => return Err(format!("MQCONN failed: completion code {comp_code}, reason {reason}").into()),
    }

    // SAFETY: hconn is the live connection returned by the successful MQCONN above.
    unsafe {
        mq::MQDISC(&mut hconn, &mut comp_code, &mut reason);
    }
    match comp_code {
        mq::MQCC_OK => {}
        mq::MQCC_WARNING => eprintln!("MQDISC completed with warning: reason {reason}"),
        _ => return Err(format!("MQDISC failed: completion code {comp_code}, reason {reason}").into()),
    }

    Ok(())
}
```

Feature flags
-------------

| Feature | Default | Description |
|---------|---------|-------------|
| `link` | ✔ | Link the MQ library at build time. Disable default features to use runtime loading instead. |
| `bindgen` | | Generate bindings from the installed MQ headers; requires `MQ_HOME`, `libclang`, and a C compiler. |
| `dlopen2` | | Load the MQ library at runtime using [`dlopen2`](https://crates.io/crates/dlopen2). |
| `mqai` | | Expose the MQAI functions. |
| `pcf` | | Expose the PCF structures. |
| `exits` | | Expose the exit structures. |
| `mqc_*` | `mqc_9_3_0_0` | Select the minimum MQ client API version, e.g. `mqc_9_3_1_0`. |
| `mqc_latest` | | Enable features of the latest MQ client API version. |

Related crates
--------------

For structure defaults and typed constants, see the companion
[`libmqm-default`](libmqm-default/README.md) and
[`libmqm-constants`](libmqm-constants/README.md) crates.

Contributions
-------------

All feedback, suggestions, contributions and enhancements are welcome. See
[CONTRIBUTING.md](CONTRIBUTING.md) for how to build, test and regenerate the bindings.
Contributors are expected to follow the [Code of Conduct](CODE_OF_CONDUCT.md).

Support
-------

This project is maintained on a best-effort basis. Maintainers intend to address security
vulnerabilities in the latest published release of each crate and on the `develop` branch,
as described in [SECURITY.md](SECURITY.md). There is no guaranteed response time or
compatibility with future versions; the API may change based on feedback and enhancements.
Older releases do not receive security fixes, so users should upgrade to the latest release.

There is no guaranteed general support or update schedule.

Report bugs and feature requests with the issue templates. Report security issues privately, as
described in [SECURITY.md](SECURITY.md).

**This crate is not approved, endorsed, acknowledged, or supported by IBM. You cannot use
IBM formal support channels (Cases/PMRs) for assistance on the use of this crate.**

License
-------

Licensed under

- Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
