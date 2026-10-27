/*!
Typed constants for the IBM® MQ Interface (MQI), Programmable Command Format (PCF), and
MQ Administration Interface (MQAI).

Use these constants with [`libmqm-sys`](https://docs.rs/libmqm-sys) when calling the raw MQ
API, or with the safe [`mqi`](https://github.com/advantic-au/mqi) crate for application code.
The constants are committed with the crate, so normal builds do not need an MQ installation
or headers.

## Example

Use typed constants and retrieve their MQ names with [`lookup::HasMqNames`]:

```
use libmqm_constants::{constants, lookup::HasMqNames as _};

let reason = constants::MQRC_NONE;
assert_eq!(reason.to_string(), "MQRC_NONE");
assert_eq!(reason.mq_primary_name(), Some("MQRC_NONE"));
```

Enable the `mqai`, `pcf`, or `exits` features to include constants for those APIs.
*/

#![cfg_attr(feature = "docsrs", doc = document_features::document_features!())]

/*!
Select the minimum MQ client API version with the `mqc_*` features.
*/

mod generated;
pub mod lookup;

pub(crate) mod bitflags;
pub(crate) mod value;

pub use generated::constants;
pub use generated::mapping;
pub use generated::types;

mod impl_types;
