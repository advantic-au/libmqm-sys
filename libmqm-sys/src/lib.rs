/*!

Bindings to the IBM® MQ Interface (MQI), Programmable Command Format (PCF) and MQ Administration Interface (MQAI) C libraries.

## Overview

- Connect to IBM MQ and send or receive messages through the MQI functions.
- Administer IBM MQ through PCF structures and MQAI functions.
- Create, send, and receive PCF messages.
- Develop MQ exit programs (untested).

For application development, start with the safe [`mqi`](https://github.com/advantic-au/mqi) crate.
Use `libmqm_sys` for direct access to the MQ C API or when building a safe wrapper.

## Linking and installation

Compile-time linking is enabled by default. Runtime loading is available with the `dlopen2`
feature, used with `--no-default-features`. Download the
[MQI library](https://ibm.biz/mq94redistclients) directly from IBM.

Install the client in `/opt/mqm` or another location and set `MQ_HOME` for a non-default
installation. The MQ shared library must be discoverable at build time for compile-time
linking and at runtime (`LD_LIBRARY_PATH` on Linux, `DYLD_LIBRARY_PATH` on macOS, or `PATH`
on Windows). Normal builds use the committed bindings and do not require MQ headers.
Generating bindings from headers additionally requires `libclang` and a C compiler.

## Raw FFI example

This example checks completion codes and reports reason codes. A warning can still leave
a valid connection, so the example reports it and proceeds to disconnect. For application
code, prefer `mqi`.

```no_run
use std::error::Error;

use libmqm_sys as mq;

fn main() -> Result<(), Box<dyn Error>> {
    let mut hconn = mq::MQHC_DEF_HCONN;
    let mut comp_code = mq::MQCC_UNKNOWN;
    let mut reason = mq::MQRC_NONE;
    let qmgr: mq::MQCHAR48 = [32; 48];

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
