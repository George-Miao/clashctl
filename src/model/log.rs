use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(
    feature = "interactive",
    derive(strum::EnumString, strum::Display, strum::EnumVariantNames),
    strum(ascii_case_insensitive, serialize_all = "UPPERCASE")
)]
pub enum Level {
    Error,
    #[cfg_attr(feature = "interactive", strum(serialize = "WARN"))]
    Warning,
    Info,
    Debug,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Log {
    #[serde(rename = "type")]
    pub log_type: Level,
    pub payload: String,
}
