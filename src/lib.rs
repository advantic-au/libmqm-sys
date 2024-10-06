#![deny(clippy::all)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::unused_async)]
#![warn(unsafe_op_in_unsafe_fn)]
#![allow(clippy::similar_names)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(feature = "bindgen")]
pub mod lib {
    mod bindgen;
    pub use bindgen::*;
}

#[cfg(not(feature = "bindgen"))]
pub mod lib {
    mod pregen;
    pub use pregen::*;
}

pub mod default;
pub mod function;
pub use function::*;

#[cfg(feature = "dlopen2")]
pub mod dlopen2;

#[cfg(feature = "link_api")]
pub mod link;
