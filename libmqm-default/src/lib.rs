#![cfg_attr(any(), rustfmt::skip)]

#[cfg(feature = "defaultgen")]
include!(concat!(env!("OUT_DIR"), "/defaults.rs"));

#[cfg(not(feature = "defaultgen"))]
include!("pregen.rs");
