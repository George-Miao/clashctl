mod config;
mod connection;
mod log;
mod proxy;
mod rule;
mod traffic;

pub use self::log::*;
pub use config::*;
pub use connection::*;
pub use proxy::*;
pub use rule::*;
pub use traffic::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(
    feature = "interactive",
    derive(strum::EnumString, strum::Display, strum::EnumVariantNames),
    strum(ascii_case_insensitive, serialize_all = "lowercase")
)]
pub enum Mode {
    Global,
    Rule,
    Direct,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Delay {
    pub delay: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub struct Version {
    pub version: semver::Version,
}
