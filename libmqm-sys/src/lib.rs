/*!

Bindings to the IBM® MQ Interface (MQI), Programmable Command Format (PCF) and MQ Administration Interface (MQAI) C libraries.

Overview
--------

You can use `libmqm_sys` to:

- Connect to an IBM MQ server to send and receive MQ messages through the MQI functions
- Administer IBM MQ server through the PCF structures and MQAI functions
- Create, send, and receive PCF messages
- Develop MQ exit programs (untested)

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

3. Set the `MQ_HOME` environment variable to the installed location.

    ```bash
    MQ_HOME=/opt/mqm
    ```

4. Add the `libmqm_sys` crate to your project:

    ```sh
    cargo add libmqm_sys
    ```

5. Use the crate in your source code:

    ```rust
    use libmqm_sys as mq;
    ```

Example
-------

```no_run
use libmqm_sys as mq;

let mut hconn = mq::MQHC_DEF_HCONN;
let mut comp_code = mq::MQCC_UNKNOWN;
let mut reason = mq::MQRC_NONE;
let mut qmgr: mq::MQCHAR48 = [32; 48]; // All spaces = default qmgr

unsafe {
    mq::MQCONN(
        &qmgr,
        &mut hconn,
        &mut comp_code,
        &mut reason,
    );
    assert_eq!(reason, mq::MQRC_NONE, "MQRC");
    assert_eq!(comp_code, mq::MQCC_OK, "MQCC");
    mq::MQDISC(&mut hconn, &mut comp_code, &mut reason);
};
```

## Features

*/

#![cfg_attr(feature = "docsrs", doc = document_features::document_features!())]

/*!

Minimum MQ client can be set using the `mqc_*` features
*/

/*!

Support
-------

This crate is not approved, endorsed, acknowledged, or supported by IBM. You cannot use
IBM formal support channels (Cases/PMRs) for assistance on the use of this crate.

Documentation
-------------

Documentation of all API functions, arguments, structures, type aliases, and constants are derived
from the MQ library header files. Accuracy of the documentation is dependent on IBM supplied header
files.

 */
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, doc(hide(feature = "bindgen")))]

#[cfg(feature = "bindgen")]
#[rustfmt::skip]
#[path ="bindgen.rs"]
mod generated;

#[cfg(not(feature = "bindgen"))]
#[rustfmt::skip]
#[path = "pregen/mod.rs"]
mod generated;

pub use generated::function::*;

pub use generated::bindings::mqi::*;

pub mod version {
    /*!
       MQ library version information used in generation of the bindings
    */
    pub use super::generated::bindings::version::*;
}

#[cfg(feature = "pcf")]
pub mod pcf {
    /*!
       Bindings to the IBM MQ [`Programmable Command Formats`](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q086860_.html)
    */
    pub use super::generated::bindings::pcf::*;
}

#[cfg(feature = "mqai")]
pub mod mqai {
    /*!
       Bindings to the IBM MQ [`Administrative Interface`](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089130_.html)
    */
    pub use super::generated::bindings::mqai::*;
}

#[cfg(feature = "exits")]
pub mod exits {
    /*!
       Bindings to the IBM MQ [`exits`](https://www.ibm.com/docs/en/SSFKSJ_latest/develop/q027670_.html)
    */
    pub use super::generated::bindings::exits::*;
}

#[cfg(feature = "mock")]
pub use generated::mock;

#[cfg(feature = "constant_lookup")]
#[rustfmt::skip]
pub mod str {
    include!(concat!(env!("OUT_DIR"), "/str.rs"));
}

#[cfg(feature = "struct_defaults")]
mod default;

#[cfg(feature = "dlopen2")]
pub mod dlopen2;

#[cfg(feature = "link_api")]
pub mod link;
