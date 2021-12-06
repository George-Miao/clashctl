use std::cmp::Ordering;

use clashctl_core::model::Proxy;

use crate::{EndlessSelf, SortMethod, SortOrder};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    strum::EnumString,
    strum::Display,
    strum::EnumVariantNames,
)]
#[strum(ascii_case_insensitive)]
pub enum ProxySortBy {
    Name,
    Type,
    Delay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProxySort {
    by: ProxySortBy,
    order: SortOrder,
}

impl ProxySort {
    #[inline]
    pub fn new(by: ProxySortBy, order: SortOrder) -> Self {
        Self { by, order }
    }

    #[inline]
    pub fn by(&self) -> ProxySortBy {
        self.by
    }

    #[inline]
    pub fn order(&self) -> SortOrder {
        self.order
    }

    #[inline]
    pub fn by_type_asc() -> Self {
        Self {
            by: ProxySortBy::Type,
            order: SortOrder::Ascendant,
        }
    }

    #[inline]
    pub fn by_name_asc() -> Self {
        Self {
            by: ProxySortBy::Name,
            order: SortOrder::Ascendant,
        }
    }

    #[inline]
    pub fn by_delay_asc() -> Self {
        Self {
            by: ProxySortBy::Delay,
            order: SortOrder::Ascendant,
        }
    }

    #[inline]
    pub fn by_type_dsc() -> Self {
        Self {
            by: ProxySortBy::Type,
            order: SortOrder::Descendant,
        }
    }

    #[inline]
    pub fn by_name_dsc() -> Self {
        Self {
            by: ProxySortBy::Name,
            order: SortOrder::Descendant,
        }
    }

    #[inline]
    pub fn by_delay_dsc() -> Self {
        Self {
            by: ProxySortBy::Delay,
            order: SortOrder::Descendant,
        }
    }
}

impl EndlessSelf for ProxySort {
    fn next_self(&mut self) {
        use ProxySortBy::*;
        use SortOrder::*;

        *self = match (self.by, self.order) {
            (Name, Ascendant) => Self {
                by: Name,
                order: Descendant,
            },
            (Name, Descendant) => Self {
                by: Type,
                order: Ascendant,
            },
            (Type, Ascendant) => Self {
                by: Type,
                order: Descendant,
            },
            (Type, Descendant) => Self {
                by: Delay,
                order: Ascendant,
            },
            (Delay, Ascendant) => Self {
                by: Delay,
                order: Descendant,
            },
            (Delay, Descendant) => Self {
                by: Name,
                order: Ascendant,
            },
        }
    }
    fn prev_self(&mut self) {
        use ProxySortBy::*;
        use SortOrder::*;

        *self = match (self.by, self.order) {
            (Name, Ascendant) => Self {
                by: Delay,
                order: Descendant,
            },
            (Name, Descendant) => Self {
                by: Name,
                order: Ascendant,
            },
            (Type, Ascendant) => Self {
                by: Name,
                order: Descendant,
            },
            (Type, Descendant) => Self {
                by: Type,
                order: Ascendant,
            },
            (Delay, Ascendant) => Self {
                by: Type,
                order: Descendant,
            },
            (Delay, Descendant) => Self {
                by: Delay,
                order: Ascendant,
            },
        }
    }
}

impl ToString for ProxySort {
    fn to_string(&self) -> String {
        format!(
            "{} {}",
            self.by,
            match self.order {
                SortOrder::Ascendant => "▲",
                SortOrder::Descendant => "▼",
            }
        )
    }
}

impl Default for ProxySort {
    fn default() -> Self {
        Self::by_delay_asc()
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
