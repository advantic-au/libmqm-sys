/*!

Default structures for the IBM® MQ Interface (MQI), Programmable Command Format (PCF) and MQ Administration Interface (MQAI) libraries.

## Features
*/

#![doc = document_features::document_features!()]

/*!
 *
 * Minimum MQ client can be set using the `mqc_*` features
*/

#[cfg(feature = "generate")]
#[rustfmt::skip]
mod defaults {
    include!(concat!(env!("OUT_DIR"), "/defaults.rs"));
}

#[cfg(not(feature = "generate"))]
#[path = "pregen/mod.rs"]
mod defaults;

#[doc(inline)]
pub use defaults::*;
