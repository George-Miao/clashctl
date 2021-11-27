use std::collections::HashMap;
use std::ops::Deref;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Proxies {
    pub proxies: HashMap<String, Proxy>,
}

impl Proxies {
    pub fn normal(&self) -> impl Iterator<Item = (&String, &Proxy)> {
        self.iter().filter(|(_, x)| x.proxy_type.is_normal())
    }

    pub fn groups(&self) -> impl Iterator<Item = (&String, &Proxy)> {
        self.iter().filter(|(_, x)| x.proxy_type.is_group())
    }

    pub fn selectors(&self) -> impl Iterator<Item = (&String, &Proxy)> {
        self.iter().filter(|(_, x)| x.proxy_type.is_selector())
    }

    pub fn built_ins(&self) -> impl Iterator<Item = (&String, &Proxy)> {
        self.iter().filter(|(_, x)| x.proxy_type.is_built_in())
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct History {
    pub time: DateTime<Utc>,
    pub delay: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
#[cfg_attr(
    feature = "interactive",
    derive(strum::EnumString, strum::Display, strum::EnumVariantNames),
    strum(ascii_case_insensitive)
)]
#[cfg_attr(feature = "ui", derive(Hash))]
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

#[test]
fn test_proxies() {
    let proxy_kv = [
        (
            "test_a".to_owned(),
            Proxy {
                proxy_type: ProxyType::Direct,
                history: vec![],
                udp: false,
                all: None,
                now: None,
            },
        ),
        (
            "test_b".to_owned(),
            Proxy {
                proxy_type: ProxyType::Selector,
                history: vec![],
                udp: false,
                all: Some(vec!["test_c".into()]),
                now: Some("test_c".into()),
            },
        ),
        (
            "test_c".to_owned(),
            Proxy {
                proxy_type: ProxyType::Shadowsocks,
                history: vec![],
                udp: false,
                all: None,
                now: None,
            },
        ),
        (
            "test_d".to_owned(),
            Proxy {
                proxy_type: ProxyType::Fallback,
                history: vec![],
                udp: false,
                all: Some(vec!["test_c".into()]),
                now: Some("test_c".into()),
            },
        ),
    ];
    let proxies = Proxies {
        proxies: HashMap::from(proxy_kv),
    };
    assert_eq!(
        {
            let mut tmp = proxies.groups().map(|x| x.0).collect::<Vec<_>>();
            tmp.sort();
            tmp
        },
        vec!["test_b", "test_d"]
    );
    assert_eq!(
        proxies.built_ins().map(|x| x.0).collect::<Vec<_>>(),
        vec!["test_a"]
    );
    assert_eq!(
        proxies.normal().map(|x| x.0).collect::<Vec<_>>(),
        vec!["test_c"]
    );
}
