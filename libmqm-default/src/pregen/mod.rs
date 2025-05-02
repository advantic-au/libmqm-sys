#[cfg(feature = "generate")]
#[rustfmt::skip]
mod definitions {
    include!(concat!(env!("OUT_DIR"), "/defaults.rs"));
}

#[cfg(not(feature = "generate"))]
#[cfg_attr(all(target_os = "windows", target_arch = "x86_64"), path = "x86_64-windows-defaults.rs")]
#[cfg_attr(all(target_os = "linux", target_arch = "x86_64"), path = "x86_64-linux-defaults.rs")]
#[cfg_attr(all(target_os = "linux", target_arch = "aarch64"), path = "aarch64-linux-defaults.rs")]
#[cfg_attr(target_os = "macos", path = "any-macos-defaults.rs")]
#[rustfmt::skip]
mod definitions;

#[doc(inline)]
pub use definitions::*;
