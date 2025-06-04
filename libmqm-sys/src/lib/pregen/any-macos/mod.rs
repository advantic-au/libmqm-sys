#![allow(clippy::all)]
#![allow(clippy::pedantic)]
#![allow(clippy::nursery)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("base.rs");
include!("mqi.rs");
#[cfg(feature = "exits")]
include!("exits.rs");
#[cfg(feature = "mqai")]
include!("mqai.rs");
#[cfg(feature = "pcf")]
include!("pcf.rs");