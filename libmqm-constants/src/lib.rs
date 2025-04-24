/*!
Overview
--------
Programming constants for the IBM® MQ Interface (MQI), Programmable Command Format (PCF) and MQ Administration Interface (MQAI) libraries.

## Capabilities
- **MQI, PCF, and MQAI**: Constants covering the MQI and MQAI API's.
- **New Types**: New type definitions for all constants to enable idiomatic usage.
- **Strings**: String to constant and constant to string functions.

## Features
*/

#![doc = document_features::document_features!()]

/*!
 *
 * Minimum MQ client can be set using the `mqc_*` features
*/

mod generated;
pub mod lookup;

pub(crate) mod bitflags;
pub(crate) mod value;

pub use generated::constants;
pub use generated::mapping;
pub use generated::types;

mod impl_types;
