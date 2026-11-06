use std::{env, path::PathBuf};

fn default_home() -> &'static str {
    env::var("CARGO_CFG_WINDOWS").map_or("/opt/mqm", |_| "c:/Program Files/IBM/MQ")
}

fn inc_sub_path() -> &'static str {
    env::var("CARGO_CFG_WINDOWS").map_or("inc", |_| "tools/c/include")
}

pub fn home_path() -> PathBuf {
    PathBuf::from(env::var("MQ_HOME").unwrap_or_else(|_| default_home().to_string()))
}

pub fn mq_inc_path() -> PathBuf {
    home_path().join(inc_sub_path())
}
