use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use url::Url;

use crate::{ConSort, ProxySort, RuleSort, Server};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConfigData {
    pub servers: Vec<Server>,
    pub using: Option<Url>,
    #[serde(default)]
    pub tui: TuiConfig,
    #[serde(default)]
    pub sort: SortsConfig,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TuiConfig {
    pub log_file: Option<PathBuf>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SortsConfig {
    pub connections: ConSort,
    pub rules: RuleSort,
    pub proxies: ProxySort,
}
