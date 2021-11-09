mod config;
mod log;
mod proxy;
mod traffic;

pub use self::log::*;
pub use config::*;
pub use proxy::*;
pub use traffic::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
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
