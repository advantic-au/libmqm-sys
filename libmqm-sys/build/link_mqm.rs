use std::env;

pub fn link_lib() -> &'static str {
    env::var("CARGO_CFG_WINDOWS").map_or("mqm_r", |_| "mqm")
}

pub fn lib_path() -> &'static str {
    env::var("CARGO_CFG_WINDOWS").map_or("lib64", |_| "tools/lib64")
}
