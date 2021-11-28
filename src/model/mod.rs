use crate::mod_use;

mod_use!(config, connection, proxy, rule, traffic);

mod log;
pub use self::log::*;

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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub struct Version {
    pub version: semver::Version,
}
