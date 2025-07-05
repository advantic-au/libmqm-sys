/*!
Dynamic loading of the MQI library using dlopen2

Example
-------
 Dynamically load the `libmqm_r` library and issue an `MQCONN`

```no_run
use dlopen2::wrapper::Container;
use libmqm_sys::{self as mq, dlopen2::MqWrapper};

# fn main() -> Result<(), dlopen2::Error> {
#
// Dynamically load the libmqm_r library
let mq: Container<MqWrapper> = unsafe { Container::load("libmqm_r") }?;

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
#
# Ok(())
# }
```
*/

use ::dlopen2::wrapper::Container;

/// A dlopen2 [`WrapperApi`](::dlopen2::wrapper::WrapperApi) implementation for MQ function calls
pub use super::generated::dlopen2::MqWrapper;

/// Name of the platform dependent MQM dynamic library
pub const MQM_LIB: &str = if cfg!(windows) { "mqm.dll" } else { "libmqm_r.so" };

/// A [dlopen2] [Container] for the MQI library
pub type MqmContainer = Container<MqWrapper>;

/// Extension trait for [`MqmContainer`] to load the MQM library using dlopen2
pub trait LoadMqmExt {
    /// Loads the MQM library using the platform dependent search rules
    ///
    /// # Safety
    /// Loading the dynamic library is inherently unsafe
    ///
    /// # Errors
    /// Will return `Err` if the dynamic library could not be loaded
    unsafe fn load_mqm_default() -> Result<Self, ::dlopen2::Error>
    where
        Self: std::marker::Sized;
}

impl LoadMqmExt for MqmContainer {
    unsafe fn load_mqm_default() -> Result<Self, ::dlopen2::Error> {
        unsafe { Self::load(MQM_LIB) }
    }
}

#[cfg(test)]
mod tests {
    use dlopen2::wrapper::Container;

    use crate as mq;

    use super::*;

    #[test]
    fn mqdist_load_default() {
        let _ = unsafe { MqmContainer::load_mqm_default() }.expect("MQM library to be loaded");
    }

    #[test]
    fn mqredist_load() -> Result<(), dlopen2::Error> {
        // Dynamically load the mqm library
        let mq: Container<MqWrapper> = unsafe { Container::load(MQM_LIB) }?;

        let mut hconn = mq::MQHC_DEF_HCONN;
        let mut comp_code = mq::MQCC_UNKNOWN;
        let mut reason = mq::MQRC_NONE;
        let qmgr: mq::MQCHAR48 = [32; 48]; // All spaces
        unsafe {
            mq.MQCONN(&qmgr, &mut hconn, &mut comp_code, &mut reason);
        }

        Ok(())
    }
}
