use std::collections::HashMap;
use std::ops::Deref;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Proxies {
    pub proxies: HashMap<String, Proxy>,
}

impl Proxies {
    pub fn proxies(&self) -> impl Iterator<Item = (&String, &Proxy)> {
        self.proxies
            .iter()
            .filter(|(_, x)| x.proxy_type.is_normal())
    }

    pub fn groups(&self) -> impl Iterator<Item = (&String, &Proxy)> {
        self.proxies.iter().filter(|(_, x)| x.proxy_type.is_group())
    }

    pub fn selectors(&self) -> impl Iterator<Item = (&String, &Proxy)> {
        self.proxies
            .iter()
            .filter(|(_, x)| x.proxy_type.is_selector())
    }

    pub fn built_ins(&self) -> impl Iterator<Item = (&String, &Proxy)> {
        self.proxies
            .iter()
            .filter(|(_, x)| x.proxy_type.is_built_in())
    }
}

impl Deref for Proxies {
    type Target = HashMap<String, Proxy>;
    fn deref(&self) -> &Self::Target {
        &self.proxies
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Proxy {
    #[serde(rename = "type")]
    pub proxy_type: ProxyType,
    pub history: Vec<History>,
    pub udp: bool,

    // Only present in ProxyGroups
    pub all: Option<Vec<String>>,
    pub now: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq)]
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

impl Ord for History {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.delay.cmp(&other.delay)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
#[cfg_attr(
    feature = "cli",
    derive(strum::EnumString, strum::Display, strum::EnumVariantNames),
    strum(ascii_case_insensitive, serialize_all = "lowercase")
)]
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
        matches!(self, ProxyType::Selector)
    }

    pub fn is_group(&self) -> bool {
        matches!(
            self,
            ProxyType::Selector
                | ProxyType::URLTest
                | ProxyType::Fallback
                | ProxyType::LoadBalance
                | ProxyType::Relay
        )
    }

    pub fn is_built_in(&self) -> bool {
        matches!(self, ProxyType::Direct | ProxyType::Reject)
    }

    pub fn is_normal(&self) -> bool {
        matches!(
            self,
            ProxyType::Shadowsocks
                | ProxyType::Vmess
                | ProxyType::ShadowsocksR
                | ProxyType::Http
                | ProxyType::Snell
                | ProxyType::Trojan
                | ProxyType::Socks5
        )
    }
}
