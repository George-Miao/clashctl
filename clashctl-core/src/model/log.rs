use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(
    feature = "enum_ext",
    derive(strum::EnumString, strum::Display, strum::EnumVariantNames),
    strum(ascii_case_insensitive, serialize_all = "UPPERCASE")
)]
pub enum Level {
    Error,
    #[cfg_attr(feature = "enum_ext", strum(serialize = "WARN"))]
    Warning,
    Info,
    Debug,
    Silent,
}

// TODO Parse log
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Log {
    #[serde(rename = "type")]
    pub log_type: Level,
    pub payload: String,
}
