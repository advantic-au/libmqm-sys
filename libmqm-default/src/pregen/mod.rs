#[cfg(feature = "defaultgen")]
#[rustfmt::skip]
mod defaults {
    include!(concat!(env!("OUT_DIR"), "/defaults.rs"));
}

#[cfg(not(feature = "defaultgen"))]
#[cfg_attr(all(target_os = "windows", target_arch = "x86_64"), path = "x86_64-windows-defaults.rs")]
#[cfg_attr(all(target_os = "linux", target_arch = "x86_64"), path = "x86_64-linux-defaults.rs")]
#[cfg_attr(target_os = "macos", path = "any-macos-defaults.rs")]
#[rustfmt::skip]
mod defaults;

#[doc(inline)]
pub use defaults::*;
