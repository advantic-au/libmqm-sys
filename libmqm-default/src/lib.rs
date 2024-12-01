#![cfg_attr(any(), rustfmt::skip)]

#[cfg(feature = "defaultgen")]
#[rustfmt::skip]
mod defaults {
    include!(concat!(env!("OUT_DIR"), "/defaults.rs"));
}

#[cfg(not(feature = "defaultgen"))]
#[cfg_attr(all(target_os = "windows", target_arch = "x86_64"), path = "x86_64-windows-pregen.rs")]
#[cfg_attr(all(target_os = "linux", target_arch = "x86_64"), path = "x86_64-linux-pregen.rs")]
#[cfg_attr(target_os = "macos", path = "any-macos-pregen.rs")]
#[rustfmt::skip]
mod defaults;

#[doc(inline)]
pub use defaults::*;