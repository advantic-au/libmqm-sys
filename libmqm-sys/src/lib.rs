/*!

Bindings to the IBM® MQ Interface (MQI) and MQ Administration Interface (MQAI) C libraries.

Overview
--------

You can use `libmqm_sys` to:

- Connect to an IBM MQ server to send and receive MQ messages through the MQI functions
- Administer IBM MQ server through the MQAI functions

Compile time dynamic linking and run-time dynamic linking is supported.

Developers must download the [MQI library](https://ibm.biz/mq94redistclients) directly from IBM.
Refer to the [Usage](#usage) instructions.

Safety
------

Functions provided in this crate are the raw `unsafe` functions exposed from the
IBM provided library. Developers should build safe rust API wrappers over these functions.
Developers who do not want to use the unsafe API should use the
[mqi](https://github.com/advantic-au/mqi) crate for a *safe* API over the MQI.

Usage
-----

1. Download and install the redistributable client from IBM:
  <https://ibm.biz/mq94redistclients>

2. Install the client in `/opt/mqm` or another location.

3. Set the MQ_HOME environment variable to the installed location.

    ```bash
    MQ_HOME=/opt/mqm
    ```

4. Add the `libmqm_sys` crate to your project:

    ```sh
    cargo add libmqm_sys
    ```

5. Use the crate in your source code:

    ```rust
    use libmqm_sys as mqsys;
    ```

Example
-------

```no_run
use std::ptr::addr_of_mut;
use libmqm_sys::lib;

let mut hconn = lib::MQHC_DEF_HCONN;
let mut comp_code = lib::MQCC_UNKNOWN;
let mut reason = lib::MQRC_NONE;
let mut qmgr: [lib::MQCHAR; 48] = [32; 48]; // All spaces = default qmgr

unsafe {
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
```

## Features

*/

#![doc = document_features::document_features!()]

/*!
 *
 * Minimum MQ client can be set using features from `mqc_9_2_0_0` to `mqc_9_4_1_0`
*/

#[cfg(feature = "bindgen")]
pub mod lib {
    //! Constants, types and structures generated by bindgen from the MQ client C library
    mod bindgen;
    #[doc(inline)]
    pub use bindgen::*;
}

#[cfg(not(feature = "bindgen"))]
pub mod lib {
    //! Constants, types and structures generated by bindgen from the MQ client C library
    mod pregen;
    #[doc(inline)]
    pub use pregen::*;
}

#[cfg(not(feature = "versiongen"))]
#[allow(clippy::unreadable_literal)]
pub mod version {
    mod pregen;
    #[doc(inline)]
    pub use pregen::*;
}

#[cfg(feature = "versiongen")]
#[allow(clippy::unreadable_literal)]
pub mod version {
    include!(concat!(env!("OUT_DIR"), "/version.rs"));
}

mod default;

mod function;
#[doc(inline)]
pub use function::*;

#[cfg(feature = "dlopen2")]
pub mod dlopen2;

#[cfg(feature = "link_api")]
pub mod link;
