use std::collections::HashMap;
use std::ops::Deref;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Proxies {
    pub proxies: HashMap<String, Proxy>,
}

impl Deref for Proxies {
    type Target = HashMap<String, Proxy>;
    fn deref(&self) -> &Self::Target {
        &self.proxies
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Proxy {
    #[serde(rename = "type")]
    pub proxy_type: ProxyType,
    pub history: Vec<History>,
    pub udp: bool,

    // Only present in Selector & URLTest
    pub all: Option<Vec<String>>,
    pub now: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct History {
    time: DateTime<Utc>,
    delay: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ProxyType {
    // Built-In types
    Direct,
    Reject,
    // ProxyGroups
    Selector,
    URLTest,
    Fallback,
    LoadBalance,
    // Proxies
    Shadowsocks,
    Vmess,
    ShadowsocksR,
    Http,
    Snell,
    Trojan,
    Socks5,
    // Relay
    Relay,
    // Unknown
    #[serde(other)]
    Unknown,
}

impl ProxyType {
    pub fn is_selector(&self) -> bool {
        match self {
            ProxyType::Selector => true,
            _ => false,
        }
    }

    pub fn is_group(&self) -> bool {
        match self {
            ProxyType::Selector
            | ProxyType::URLTest
            | ProxyType::Fallback
            | ProxyType::LoadBalance
            | ProxyType::Relay => true,
            _ => false,
        }
    }

    pub fn is_built_in(&self) -> bool {
        match self {
            ProxyType::Direct | ProxyType::Reject => true,
            _ => false,
        }
    }

    pub fn is_normal(&self) -> bool {
        match self {
            ProxyType::Shadowsocks
            | ProxyType::Vmess
            | ProxyType::ShadowsocksR
            | ProxyType::Http
            | ProxyType::Snell
            | ProxyType::Trojan
            | ProxyType::Socks5 => true,
            _ => false,
        }
    }
}
