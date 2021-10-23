use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Deref;

use chrono::{DateTime, Local};
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

#[derive(Serialize, Deserialize, Debug, Eq, Ord)]
pub struct History {
    pub time: DateTime<Local>,
    pub delay: u64,
}

impl PartialEq for History {
    fn eq(&self, other: &Self) -> bool {
        self.delay.eq(&other.delay)
    }
}

impl PartialOrd for History {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.delay.partial_cmp(&other.delay)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Eq, Ord)]
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

impl Display for ProxyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
