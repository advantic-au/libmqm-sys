#![allow(clippy::all)]
#![allow(clippy::pedantic)]
#![allow(clippy::nursery)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod bindings {
    include!(concat!(env!("OUT_DIR"), "/mqi.rs"));
    #[cfg(feature = "exits")]
    include!(concat!(env!("OUT_DIR"), "/exits.rs"));
    #[cfg(feature = "mqai")]
    include!(concat!(env!("OUT_DIR"), "/mqai.rs"));
    #[cfg(feature = "pcf")]
    include!(concat!(env!("OUT_DIR"), "/pcf.rs"));

    pub mod version {
        include!(concat!(env!("OUT_DIR"), "/version.rs"));
    }
}

pub mod function {
    include!(concat!(env!("OUT_DIR"), "/function.rs"));
}

#[cfg(feature = "dlopen2")]
pub mod dlopen2 {
    include!(concat!(env!("OUT_DIR"), "/dlopen2.rs"));
}

#[cfg(feature = "mock")]
pub mod mock {
    include!(concat!(env!("OUT_DIR"), "/mock.rs"));
}

#[cfg(feature = "link_api")]
pub mod link {
    include!(concat!(env!("OUT_DIR"), "/link.rs"));
}
