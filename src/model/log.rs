use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    Error,
    Warning,
    Info,
    Debug,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Log {
    #[serde(rename = "type")]
    log_type: Level,
    payload: String,
}
