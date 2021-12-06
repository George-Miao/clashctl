use crate::{EndlessSelf, OrderBy, SortMethod, SortOrder};

use clashctl_core::model::Rule;

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
pub enum RuleSortBy {
    Payload,
    Proxy,
    Type,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RuleSort {
    by: RuleSortBy,
    order: SortOrder,
}

impl RuleSort {
    #[inline]
    pub fn new(by: RuleSortBy, order: SortOrder) -> Self {
        Self { by, order }
    }

    #[inline]
    pub fn by(&self) -> RuleSortBy {
        self.by
    }

    #[inline]
    pub fn order(&self) -> SortOrder {
        self.order
    }

    #[inline]
    pub fn by_type_asc() -> Self {
        Self::new(RuleSortBy::Type, SortOrder::Ascendant)
    }

    #[inline]
    pub fn by_type_dsc() -> Self {
        Self::new(RuleSortBy::Type, SortOrder::Descendant)
    }

    #[inline]
    pub fn by_payload_asc() -> Self {
        Self::new(RuleSortBy::Payload, SortOrder::Ascendant)
    }

    #[inline]
    pub fn by_payload_dsc() -> Self {
        Self::new(RuleSortBy::Payload, SortOrder::Descendant)
    }

    #[inline]
    pub fn by_proxy_name_asc() -> Self {
        Self::new(RuleSortBy::Proxy, SortOrder::Ascendant)
    }

    #[inline]
    pub fn by_proxy_name_dsc() -> Self {
        Self::new(RuleSortBy::Proxy, SortOrder::Descendant)
    }
}

impl EndlessSelf for RuleSort {
    fn next_self(&mut self) {
        use RuleSortBy::*;
        use SortOrder::*;

        *self = match (self.by, self.order) {
            (Payload, Ascendant) => Self::by_payload_dsc(),
            (Payload, Descendant) => Self::by_type_asc(),
            (Type, Ascendant) => Self::by_type_dsc(),
            (Type, Descendant) => Self::by_proxy_name_asc(),
            (Proxy, Ascendant) => Self::by_proxy_name_dsc(),
            (Proxy, Descendant) => Self::by_payload_asc(),
        }
    }
    fn prev_self(&mut self) {
        use RuleSortBy::*;
        use SortOrder::*;

        *self = match (self.by, self.order) {
            (Payload, Ascendant) => Self::by_proxy_name_dsc(),
            (Payload, Descendant) => Self::by_payload_asc(),
            (Type, Ascendant) => Self::by_payload_dsc(),
            (Type, Descendant) => Self::by_type_asc(),
            (Proxy, Ascendant) => Self::by_type_dsc(),
            (Proxy, Descendant) => Self::by_proxy_name_asc(),
        }
    }
}

impl ToString for RuleSort {
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

impl SortMethod<Rule> for RuleSort {
    fn sort_fn(&self, a: &Rule, b: &Rule) -> std::cmp::Ordering {
        match self.by {
            RuleSortBy::Payload => a.payload.cmp(&b.payload),
            RuleSortBy::Proxy => a.proxy.cmp(&b.proxy),
            RuleSortBy::Type => a.rule_type.cmp(&b.rule_type),
        }
        .order_by(self.order)
    }
}

impl Default for RuleSort {
    fn default() -> Self {
        Self::by_payload_dsc()
    }
}
