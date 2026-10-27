/*!
Default initializers for IBM® MQ C structures used by the MQI, Programmable Command Format
(PCF), and MQ Administration Interface (MQAI).

Use these values when calling the raw functions from
[`libmqm-sys`](https://docs.rs/libmqm-sys). For application code, the safe
[`mqi`](https://github.com/advantic-au/mqi) crate provides a higher-level API.
Structure defaults are committed with the crate, so normal builds do not need MQ headers.

## Example

Initialize an MQ message descriptor with the MQ-defined defaults:

```
let mut message_descriptor = libmqm_default::MQMD2_DEFAULT;
assert_eq!(message_descriptor.Version, libmqm_sys::MQMD_VERSION_2);
```

Enable `pcf` or `exits` to include defaults for those APIs.
*/

#![cfg_attr(feature = "docsrs", doc = document_features::document_features!())]

/*!
Select the minimum MQ client API version with the `mqc_*` features.
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
        assert_eq!(defaults::MQMD2_DEFAULT.Encoding, libmqm_sys::MQENC_NATIVE);
        assert_eq!(defaults::MQMD2_DEFAULT.Version, libmqm_sys::MQMD_VERSION_2);
    }
}
