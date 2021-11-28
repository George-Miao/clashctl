use std::cmp::Ordering;

use either::Either;
use owo_colors::OwoColorize;
use terminal_size::{terminal_size, Height, Width};

use crate::model::{Proxies, Proxy};
use crate::{
    cli::ProxyListOpt,
    interactive::{ProxySort, ProxySortBy, SortMethod},
};
use crate::{
    interactive::{SortOrder, Sortable},
    Result,
};

impl Proxies {
    pub fn names(&self) -> impl Iterator<Item = &String> {
        self.iter().map(|x| x.0)
    }

    pub fn render_list(&self, opt: &ProxyListOpt) -> Result<()> {
        let (Width(terminal_width), _) = terminal_size().unwrap_or((Width(70), Height(0)));
        println!("\n{:-<1$}", "", terminal_width as usize);
        println!("{:<18}{:<8}NAME", "TYPE", "DELAY");
        println!("{:-<1$}", "", terminal_width as usize);

        if opt.plain {
            self.render_plain(opt);
        } else {
            self.render_tree(opt)
        }

        println!("{:-<1$}", "", terminal_width as usize);
        Ok(())
    }

    fn render_plain(&self, opt: &ProxyListOpt) {
        let mut list = self.iter().collect::<Vec<_>>();
        let sort_method = ProxySort::new(opt.sort_by, opt.sort_order);

        list.sort_with(&sort_method);

        let iter = if opt.reverse {
            Either::Left(list.into_iter().rev())
        } else {
            Either::Right(list.into_iter())
        }
        .filter(|x| {
            let proxy_type = &x.1.proxy_type;
            // When include all types
            if opt.include.is_empty() {
                !opt.exclude.contains(proxy_type)
            } else {
                // When types included is specified
                opt.include.contains(proxy_type)
            }
        });

        for (name, proxy) in iter {
            let delay = proxy
                .history
                .get(0)
                .map(|x| match x.delay {
                    0 => "?".to_owned(),
                    delay => delay.to_string(),
                })
                .unwrap_or_else(|| "-".into());
            let type_name = proxy.proxy_type.to_string();
            println!("{:<18}{:<8}{}", type_name.green(), delay, name)
        }
    }

    fn render_tree(&self, opt: &ProxyListOpt) {
        let list = self
            .iter()
            .filter(|x| {
                let proxy_type = x.1.proxy_type;
                proxy_type.is_group() && !opt.exclude.contains(&proxy_type)
            })
            .collect::<Vec<_>>();

        let groups = if opt.reverse {
            Either::Left(list.iter().rev())
        } else {
            Either::Right(list.iter())
        };

        let sort_method = ProxySort::new(opt.sort_by, opt.sort_order);

        for (name, group) in groups.into_iter() {
            // Since list only contains groups, and only groups have `all`, so it is safe to [`unwrap`]
            println!("{:<16}  -       {}\n", group.proxy_type.blue(), name);
            let mut members = group
                .all
                .as_ref()
                .expect("Proxy groups should have `all`")
                .iter()
                .map(|member_name| self.iter().find(|(name, _)| &member_name == name).unwrap())
                .collect::<Vec<_>>();
            members.sort_with(&sort_method);
            for (
                name,
                Proxy {
                    proxy_type,
                    history,
                    ..
                },
            ) in members
            {
                let delay = history
                    .get(0)
                    .map(|x| match x.delay {
                        0 => "?".to_owned(),
                        delay => delay.to_string(),
                    })
                    .unwrap_or_else(|| "-".into());
                println!("  {:<16}{:<8}{}", proxy_type.green(), delay, name)
            }
            println!();
        }
    }
}

impl SortMethod<(&String, &Proxy)> for ProxySort {
    fn sort_fn(&self, a: &(&String, &Proxy), b: &(&String, &Proxy)) -> Ordering {
        let ret = match self.by() {
            ProxySortBy::Type => a.1.proxy_type.cmp(&b.1.proxy_type),
            ProxySortBy::Name => a.0.cmp(b.0),
            ProxySortBy::Delay => match (a.1.history.get(0), b.1.history.get(0)) {
                // 0 delay means unable to connect, so handle exceptionally
                // This will push all 0-delay proxies to the end of list
                (Some(l_history), Some(r_history)) => match (l_history.delay, r_history.delay) {
                    (0, 0) => Ordering::Equal,
                    (0, _) => Ordering::Greater,
                    (_, 0) => Ordering::Less,
                    (a, b) => a.cmp(&b),
                },
                (Some(_), None) => Ordering::Greater,
                (None, Some(_)) => Ordering::Less,
                _ => Ordering::Equal,
            },
        };
        match self.order() {
            SortOrder::Ascendant => ret,
            SortOrder::Descendant => ret.reverse(),
        }
    }
}
