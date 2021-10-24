use std::cmp::Ordering;

use owo_colors::OwoColorize;
use terminal_size::{terminal_size, Height, Width};

use crate::model::{Proxies, Proxy};
use crate::Result;

impl Proxies {
    pub fn render_list(&self, sort: ProxySort) -> Result<()> {
        let (Width(terminal_width), _) = terminal_size().unwrap_or((Width(70), Height(0)));
        let mut list = self.iter().collect::<Vec<_>>();
        sort.sort(&mut list);
        println!("\n{:-<1$}", "", terminal_width as usize);
        println!("{:<16}{:<8}{}", "TYPE", "DELAY", "NAME");
        println!("{:-<1$}", "", terminal_width as usize);
        for (name, _) in list.into_iter() {
            let proxy = self.get(name).unwrap();
            let delay = proxy
                .history
                .iter()
                .next()
                .and_then(|x| Some(format!("{}", x.delay)))
                .unwrap_or_else(|| "-".into());
            let type_name = proxy.proxy_type.to_string();
            println!("{:<16}{:<8}{}", type_name.green(), delay, name)
        }
        println!("{:-<1$}", "", terminal_width as usize);
        Ok(())
    }
}

pub enum ProxySort {
    Type,
    Name,
    Delay,
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
                (Some(l_history), Some(r_history)) => l_history.cmp(&r_history),
                (Some(_), None) => Ordering::Greater,
                (None, Some(_)) => Ordering::Less,
                _ => Ordering::Equal,
            },
        });
    }
}
