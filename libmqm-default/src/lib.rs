/*!

Default structures for the IBM® MQ Interface (MQI), Programmable Command Format (PCF) and MQ Administration Interface (MQAI) libraries.

## Features
*/

#![cfg_attr(feature = "docsrs", doc = document_features::document_features!())]

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
#[rustfmt::skip]
mod defaults;

#[doc(inline)]
pub use defaults::*;

#[cfg(test)]
mod tests {
    use crate::defaults;

    #[test]
    fn endian() {
        assert_eq!(defaults::MQMD2_DEFAULT.Encoding, libmqm_sys::lib::MQENC_NATIVE);
        assert_eq!(defaults::MQMD2_DEFAULT.Version, libmqm_sys::lib::MQMD_VERSION_2);
    }
}
