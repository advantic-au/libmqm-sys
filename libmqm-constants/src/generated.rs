#[cfg(feature = "generate")]
mod c {
    #![allow(
        non_upper_case_globals,
        clippy::unreadable_literal,
        clippy::needless_raw_string_hashes,
        clippy::upper_case_acronyms
    )]

    pub mod mapping {
        // This file is generated during the build process
        include!(concat!(env!("OUT_DIR"), "/mapping.rs"));
    }
    include!(concat!(env!("OUT_DIR"), "/new_types.rs"));
}

#[cfg(all(not(feature = "generate"), target_os = "windows", target_arch = "x86_64"))]
mod c {
    #![allow(
        non_upper_case_globals,
        clippy::unreadable_literal,
        clippy::needless_raw_string_hashes,
        clippy::upper_case_acronyms
    )]

    pub mod mapping {
        include!("pregen/x86_64-windows-mapping.rs");
    }
    include!("pregen/x86_64-windows-new_types.rs");
}

#[cfg(all(not(feature = "generate"), target_os = "linux", target_arch = "x86_64"))]
mod c {
    #![allow(
        non_upper_case_globals,
        clippy::unreadable_literal,
        clippy::needless_raw_string_hashes,
        clippy::upper_case_acronyms
    )]

    pub mod mapping {
        include!("pregen/x86_64-linux-mapping.rs");
    }
    include!("pregen/x86_64-linux-new_types.rs");
}

#[cfg(all(not(feature = "generate"), target_os = "linux", target_arch = "aarch64"))]
mod c {
    #![allow(
        non_upper_case_globals,
        clippy::unreadable_literal,
        clippy::needless_raw_string_hashes,
        clippy::upper_case_acronyms
    )]

    pub mod mapping {
        include!("pregen/aarch64-linux-mapping.rs");
    }
    include!("pregen/aarch64-linux-new_types.rs");
}

#[cfg(all(not(feature = "generate"), target_os = "linux", target_arch = "powerpc64"))]
mod c {
    #![allow(
        non_upper_case_globals,
        clippy::unreadable_literal,
        clippy::needless_raw_string_hashes,
        clippy::upper_case_acronyms
    )]

    pub mod mapping {
        include!("pregen/powerpc64-linux-mapping.rs");
    }
    include!("pregen/powerpc64-linux-new_types.rs");
}

#[cfg(all(not(feature = "generate"), target_os = "linux", target_arch = "s390x"))]
mod c {
    #![allow(
        non_upper_case_globals,
        clippy::unreadable_literal,
        clippy::needless_raw_string_hashes,
        clippy::upper_case_acronyms
    )]

    pub mod mapping {
        include!("pregen/s390x-linux-mapping.rs");
    }
    include!("pregen/s390x-linux-new_types.rs");
}

#[cfg(all(not(feature = "generate"), target_os = "macos"))]
mod c {
    #![allow(
        non_upper_case_globals,
        clippy::unreadable_literal,
        clippy::needless_raw_string_hashes,
        clippy::upper_case_acronyms
    )]

    pub mod mapping {
        include!("pregen/any-macos-mapping.rs");
    }
    include!("pregen/any-macos-new_types.rs");
}

pub mod constants {
    pub use super::c::constants::*;

    #[cfg(feature = "mqai")]
    mod mqai {
        use crate::types;
        use libmqm_sys::lib as sys;

        pub const MQBA_FIRST: types::Selector = types::Selector(sys::MQBA_FIRST);
        pub const MQBA_LAST: types::Selector = types::Selector(sys::MQBA_LAST);
        pub const MQGA_FIRST: types::Selector = types::Selector(sys::MQGA_FIRST);
        pub const MQGA_LAST: types::Selector = types::Selector(sys::MQGA_LAST);
        pub const MQOA_FIRST: types::Selector = types::Selector(sys::MQOA_FIRST);
        pub const MQOA_LAST: types::Selector = types::Selector(sys::MQOA_LAST);
        pub const MQUA_FIRST: types::Selector = types::Selector(sys::MQUA_FIRST);
        pub const MQUA_LAST: types::Selector = types::Selector(sys::MQUA_LAST);
    }
    #[cfg(feature = "mqai")]
    pub use mqai::*;
}

pub mod mapping {
    pub use super::c::mapping::*;

    use crate::lookup::{BinarySearchSource, ConstSource, PhfSource};

    type MqxaSource<'a> = ConstSource<BinarySearchSource<'a>, BinarySearchSource<'a>>;

    pub const MQXA_MAPSTR: MqxaSource = ConstSource(MQIA_MAPSTR, MQCA_MAPSTR);
    pub const MQRC_FULL_MAPSTR: ConstSource<PhfSource, PhfSource> = ConstSource(MQRC_MAPSTR, MQRCCF_MAPSTR);

    pub struct SelectorLookup;
}

pub mod types {
    use crate::value::{define_new_type, impl_value};
    use ::libmqm_sys::lib as sys;

    pub use super::c::types::*;

    define_new_type!(pub MQXA, sys::MQLONG, super::mapping::MQXA_MAPSTR, "Selectors for MQIA and MQCA");
    impl_value!(MQXA, sys::MQLONG);
    define_new_type!(pub MQRC, sys::MQLONG, super::mapping::MQRC_FULL_MAPSTR, "Reason Code from an MQ function call");
    impl_value!(MQRC, sys::MQLONG);

    define_new_type!(pub Selector, sys::MQLONG, super::mapping::SelectorLookup, "All Selectors including [`MQIA`], [`MQCA`], [`MQIACF`], [`MQCACF`], [`MQIACH`], [`MQCACH`], [`MQIASY`] and [`MQHA`]");
    impl_value!(Selector, sys::MQLONG);
}
