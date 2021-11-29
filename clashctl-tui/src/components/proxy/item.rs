use std::fmt::Debug;

use crate::clashctl::model::{History, Proxy, ProxyType};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProxyItem {
    pub(super) name: String,
    pub(super) proxy_type: ProxyType,
    pub(super) history: Option<History>,
    pub(super) udp: bool,
    pub(super) now: Option<String>,
}

impl<'a> From<(&'a str, &'a Proxy)> for ProxyItem {
    fn from(val: (&'a str, &'a Proxy)) -> Self {
        let (name, proxy) = val;
        Self {
            name: name.to_owned(),
            proxy_type: proxy.proxy_type,
            history: proxy.history.get(0).cloned(),
            udp: proxy.udp,
            now: proxy.now.as_ref().map(Into::into),
        }
    }
}

impl ProxyItem {
    pub fn proxy_type(&self) -> ProxyType {
        self.proxy_type
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn delay(&self) -> Option<u64> {
        self.history.as_ref().map(|x| x.delay)
    }
}
