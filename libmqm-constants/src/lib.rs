//! Overview
//! --------
//! Programming constants for the IBM® MQ Interface (MQI) and MQ Administration Interface (MQAI) libraries.

//! Overview
//! ------------
//!
//! The `libmqm-constants` crate provides the a comprehensive definition of IBM MQ constants.  
//!
//! ## Key Features
//! - **MQI and MQAI**: Constants covering the MQI and MQAI API's.
//! - **New Types**: New type definitions for all constants to enable idiomatic usage.
//! - **Strings**: String to constant and constant to string functions.

mod generated;
pub mod lookup;

pub(crate) mod bitflags;
pub(crate) mod value;

pub use generated::constants;
pub use generated::mapping;
pub use generated::types;

mod impl_types;
