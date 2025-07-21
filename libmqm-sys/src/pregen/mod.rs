#![allow(clippy::missing_safety_doc, clippy::too_many_arguments, non_snake_case, non_camel_case_types, clippy::unreadable_literal, clippy::doc_markdown)]

#[cfg_attr(all(target_os = "windows", target_arch = "x86_64"), path = "x86_64-windows/mod.rs")]
#[cfg_attr(all(target_os = "linux", target_arch = "x86_64"), path = "x86_64-linux/mod.rs")]
#[cfg_attr(all(target_os = "linux", target_arch = "aarch64"), path = "aarch64-linux/mod.rs")]
#[cfg_attr(all(target_os = "linux", target_arch = "powerpc64"), path = "powerpc64-linux/mod.rs")]
#[cfg_attr(all(target_os = "linux", target_arch = "s390x"), path = "s390x-linux/mod.rs")]
#[cfg_attr(target_os = "macos", path = "any-macos/mod.rs")]
pub mod bindings;

pub mod function;
#[cfg(feature = "dlopen2")]
pub mod dlopen2;

#[cfg(feature = "mock")]
#[allow(warnings)] // reason = no control on generated code
pub mod mock;

#[cfg(feature = "link_api")]
pub mod link;

#[cfg(feature = "exits")]
mod iep;