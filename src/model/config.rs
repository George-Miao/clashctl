use serde::{Deserialize, Serialize};

use crate::model::{Level, Mode};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    port: u64,
    socks_port: u64,
    redir_port: u64,
    tproxy_port: u64,
    mixed_port: u64,
    allow_lan: bool,
    ipv6: bool,
    mode: Mode,
    log_level: Level,
    bind_address: String,
    authentication: Vec<String>,
}
