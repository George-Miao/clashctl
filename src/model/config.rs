use serde::{Deserialize, Serialize};

use crate::model::{Level, Mode};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub port: u64,
    pub socks_port: u64,
    pub redir_port: u64,
    pub tproxy_port: u64,
    pub mixed_port: u64,
    pub allow_lan: bool,
    pub ipv6: bool,
    pub mode: Mode,
    pub log_level: Level,
    pub bind_address: String,
    pub authentication: Vec<String>,
}
