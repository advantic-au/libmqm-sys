/*!

Default structures for the IBM® MQ Interface (MQI) and MQ Administration Interface (MQAI) libraries.

*/

#[cfg(feature = "defaultgen")]
#[rustfmt::skip]
mod defaults {
    include!(concat!(env!("OUT_DIR"), "/defaults.rs"));
}

#[cfg(not(feature = "defaultgen"))]
#[path = "pregen/mod.rs"]
mod defaults;

#[doc(inline)]
pub use defaults::*;
