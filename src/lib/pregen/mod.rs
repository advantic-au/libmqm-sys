#![allow(clippy::all)]
#![allow(clippy::pedantic)]
#![allow(clippy::nursery)]

#[cfg_attr(all(target_os="windows", target_arch="x86_64"), path = "x86_64-windows-bindings.rs")]
#[cfg_attr(all(target_os="linux",   target_arch="x86_64"), path = "x86_64-linux-bindings.rs")]
mod bindings;
pub use bindings::*;
