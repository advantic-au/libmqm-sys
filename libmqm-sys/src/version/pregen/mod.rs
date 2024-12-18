#[cfg_attr(all(target_os = "windows", target_arch = "x86_64"), path = "x86_64-windows-version.rs")]
#[cfg_attr(all(target_os = "linux", target_arch = "x86_64"), path = "x86_64-linux-version.rs")]
#[cfg_attr(target_os = "macos", path = "any-macos-version.rs")]
mod compile;
#[doc(inline)]
pub use compile::*;