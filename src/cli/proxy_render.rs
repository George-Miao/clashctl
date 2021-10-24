use std::cmp::Ordering;
use std::str::FromStr;

// use clap::Parser;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use terminal_size::{terminal_size, Height, Width};

use crate::cli::ProxyListOpt;
use crate::model::{Proxies, Proxy};
use crate::{Error, Result};

impl Proxies {
    pub fn render_list(&self, opt: &ProxyListOpt) -> Result<()> {
        let (Width(terminal_width), _) = terminal_size().unwrap_or((Width(70), Height(0)));
        let mut list = self.iter().collect::<Vec<_>>();
        opt.sort.sort(&mut list);
        println!("\n{:-<1$}", "", terminal_width as usize);
        println!("{:<16}{:<8}{}", "TYPE", "DELAY", "NAME");
        println!("{:-<1$}", "", terminal_width as usize);
        let iter: Box<dyn Iterator<Item = _>> = if opt.reverse {
            Box::new(list.into_iter().rev())
        } else {
            Box::new(list.into_iter())
        };
        let show_all = opt.proxy_types.len() == 0;
        for (name, proxy) in iter {
            if opt.exclude.contains(&proxy.proxy_type) {
                continue;
            }
            if !show_all && !opt.proxy_types.contains(&proxy.proxy_type) {
                continue;
            }
            let delay = proxy
                .history
                .get(0)
                .map(|x| match x.delay {
                    0 => "?".to_owned(),
                    delay => delay.to_string(),
                })
                .unwrap_or_else(|| "-".into());
            let type_name = proxy.proxy_type.to_string();
            println!("{:<16}{:<8}{}", type_name.green(), delay, name)
        }
        println!("{:-<1$}", "", terminal_width as usize);
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum ProxySort {
    Type,
    Name,
    Delay,
}

impl FromStr for ProxySort {
    type Err = Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "type" => Ok(Self::Type),
            "name" => Ok(Self::Name),
            "delay" => Ok(Self::Delay),
            _ => Err(Self::Err::BadOption),
        }
    }
}

impl ProxySort {
    pub fn by_type() -> Self {
        Self::Type
    }

    pub fn by_name() -> Self {
        Self::Name
    }

    pub fn by_delay() -> Self {
        Self::Delay
    }

    pub fn sort(&self, proxies: &mut Vec<(&String, &Proxy)>) {
        proxies.sort_by(|lhs, rhs| match self {
            Self::Type => lhs.1.proxy_type.cmp(&rhs.1.proxy_type),
            Self::Name => lhs.0.cmp(&rhs.0),
            Self::Delay => match (lhs.1.history.iter().next(), rhs.1.history.iter().next()) {
                // 0 delay means unable to connect, so handle exceptionally
                // This will push all 0-delay proxies to the end of list
                (Some(l_history), Some(r_history)) => match (l_history.delay, r_history.delay) {
                    (0, 0) => Ordering::Equal,
                    (0, _) => Ordering::Greater,
                    (_, 0) => Ordering::Less,
                    (lhs, rhs) => lhs.cmp(&rhs),
                },
                (Some(_), None) => Ordering::Greater,
                (None, Some(_)) => Ordering::Less,
                _ => Ordering::Equal,
            },
        });
    }
}
