/*!

Compile time linking of the MQI library

Example
-------
Use the compile time linked MQ library and issue an `MQCONN`

```no_run
use libmqm_sys::{self as mq, Mqi as _};

// Use the compile time linked MQ library
let mq = libmqm_sys::link::LinkedMq;

// Connect to MQ
let mut hconn = mq::MQHC_DEF_HCONN;
let mut comp_code = mq::MQCC_UNKNOWN;
let mut reason = mq::MQRC_NONE;
let mut qmgr: mq::MQCHAR48 = [32; 48]; // All spaces
unsafe {
   mq.MQCONN(
     &qmgr,
     &mut hconn,
     &mut comp_code,
     &mut reason,
   );
}
```

*/

pub use super::generated::link::*;
