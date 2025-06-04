
#[cfg_attr(all(target_os = "windows", target_arch = "x86_64"), path = "x86_64-windows/mod.rs")]
#[cfg_attr(all(target_os = "linux", target_arch = "x86_64"), path = "x86_64-linux/mod.rs")]
#[cfg_attr(all(target_os = "linux", target_arch = "aarch64"), path = "aarch64-linux/mod.rs")]
#[cfg_attr(all(target_os = "linux", target_arch = "powerpc64"), path = "powerpc64-linux/mod.rs")]
#[cfg_attr(all(target_os = "linux", target_arch = "s390x"), path = "s390x-linux/mod.rs")]
#[cfg_attr(target_os = "macos", path = "any-macos")]
mod bindings;

#[doc(inline)]
pub use bindings::*;
