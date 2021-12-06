use crate::mod_use;

mod_use![config, connection, proxy, rule, traffic];

mod log;
pub use self::log::*;

use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(
    feature = "enum_ext",
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
    // Clash Premium only
    pub premium: Option<bool>,
    pub version: VersionPayload,
}

cfg_if! {
    if #[cfg(feature = "deserialize")] {
        use chrono::{Utc, DateTime};
        pub type TimeType = DateTime<Utc>;
    } else {
        pub type TimeType = String;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(untagged, rename_all = "lowercase")]
pub enum VersionPayload {
    #[cfg(feature = "deserialize")]
    SemVer(semver::Version),
    Raw(String),
}

impl ToString for VersionPayload {
    fn to_string(&self) -> String {
        match self {
            #[cfg(feature = "deserialize")]
            VersionPayload::SemVer(ver) => ver.to_string(),
            VersionPayload::Raw(content) => content.to_owned(),
        }
    }
}
