#![allow(clippy::all)]
#![allow(clippy::pedantic)]
#![allow(clippy::nursery)]


#![allow(clippy::all)]
#![allow(clippy::pedantic)]
#![allow(clippy::nursery)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/mqi.rs"));
#[cfg(feature = "exits")]
include!(concat!(env!("OUT_DIR"), "/exits.rs"));
#[cfg(feature = "mqai")]
include!(concat!(env!("OUT_DIR"), "/mqai.rs"));
#[cfg(feature = "pcf")]
include!(concat!(env!("OUT_DIR"), "/pcf.rs"));
