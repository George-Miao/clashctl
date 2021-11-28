use std::collections::HashMap;

use itertools::Itertools;
use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::Widget,
};

use crate::{
    define_widget,
    interactive::{EndlessSelf, SortMethod, SortOrder},
    model::{Rule, RuleType, Rules},
    ui::{
        components::{MovableList, MovableListItem, MovableListState},
        AsColor,
    },
};

define_widget!(RulePage);

impl<'a> Widget for RulePage<'a> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        MovableList::new("Rules", &self.state.rule_state).render(area, buf);
    }
}

impl AsColor for RuleType {
    fn as_color(&self) -> tui::style::Color {
        match self {
            RuleType::Domain => Color::Green,
            RuleType::DomainSuffix => Color::Green,
            RuleType::DomainKeyword => Color::Green,
            RuleType::GeoIP => Color::Yellow,
            RuleType::IPCIDR => Color::Yellow,
            RuleType::SrcIPCIDR => Color::Yellow,
            RuleType::SrcPort => Color::Yellow,
            RuleType::DstPort => Color::Yellow,
            RuleType::Process => Color::Yellow,
            RuleType::Match => Color::Blue,
            RuleType::Direct => Color::Blue,
            RuleType::Reject => Color::Red,
        }
    }
}

impl<'a> From<Rules> for MovableListState<'a, Rule, RuleSort> {
    fn from(val: Rules) -> Self {
        Self::new_with_sort(val.rules, RuleSort::noop())
    }
}

impl<'a> MovableListItem<'a> for Rule {
    fn to_spans(&self) -> Spans<'a> {
        let type_color = self.rule_type.as_color();
        let name_color = if self.proxy == "DIRECT" || self.proxy == "REJECT" {
            Color::DarkGray
        } else {
            Color::Yellow
        };
        let gray = Style::default().fg(Color::DarkGray);
        let r_type: &'static str = self.rule_type.into();
        let dash: String = "─".repeat(35_usize.saturating_sub(self.payload.len()) + 2) + " ";
        vec![
            Span::styled(format!("{:16}", r_type), Style::default().fg(type_color)),
            Span::styled(
                self.payload.to_owned() + " ",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::styled(dash, gray),
            Span::styled(self.proxy.to_owned(), Style::default().fg(name_color)),
        ]
        .into()
    }
}

impl Rules {
    pub fn most_frequent_proxy(&self) -> Option<&str> {
        self.frequency()
            .into_iter()
            .max_by_key(|(_, v)| *v)
            .map(|(k, _)| k)
    }

    pub fn frequency(&self) -> HashMap<&str, usize> {
        self.rules
            .iter()
            .filter(|x| x.proxy != "DIRECT" && x.proxy != "REJECT")
            .map(|x| x.proxy.as_str())
            .counts()
    }

    pub fn owned_frequency(&self) -> HashMap<String, usize> {
        self.rules
            .iter()
            .filter(|x| x.proxy != "DIRECT" && x.proxy != "REJECT")
            .map(|x| x.proxy.to_owned())
            .counts()
    }
}

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
    RuleName,
    ProxyName,
    Type,
    Noop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
        Self {
            by: RuleSortBy::Type,
            order: SortOrder::Ascendant,
        }
    }

    #[inline]
    pub fn by_type_dsc() -> Self {
        Self {
            by: RuleSortBy::Type,
            order: SortOrder::Descendant,
        }
    }

    #[inline]
    pub fn by_rule_name_asc() -> Self {
        Self {
            by: RuleSortBy::RuleName,
            order: SortOrder::Ascendant,
        }
    }

    #[inline]
    pub fn by_rule_name_dsc() -> Self {
        Self {
            by: RuleSortBy::RuleName,
            order: SortOrder::Descendant,
        }
    }

    #[inline]
    pub fn by_proxy_name_asc() -> Self {
        Self {
            by: RuleSortBy::ProxyName,
            order: SortOrder::Ascendant,
        }
    }

    #[inline]
    pub fn by_proxy_name_dsc() -> Self {
        Self {
            by: RuleSortBy::ProxyName,
            order: SortOrder::Descendant,
        }
    }

    #[inline]
    pub fn noop() -> Self {
        Self {
            by: RuleSortBy::Noop,
            order: SortOrder::Ascendant,
        }
    }
}

impl EndlessSelf for RuleSort {
    fn next_self(&mut self) {
        use RuleSortBy::*;
        use SortOrder::*;

        *self = match (self.by, self.order) {
            (RuleName, Ascendant) => Self {
                by: RuleName,
                order: Descendant,
            },
            (RuleName, Descendant) => Self {
                by: Type,
                order: Ascendant,
            },
            (Type, Ascendant) => Self {
                by: Type,
                order: Descendant,
            },
            (Type, Descendant) => Self {
                by: ProxyName,
                order: Ascendant,
            },
            (ProxyName, Ascendant) => Self {
                by: ProxyName,
                order: Descendant,
            },
            (ProxyName, Descendant) => Self {
                by: Noop,
                order: Ascendant,
            },
            (Noop, _) => Self {
                by: RuleName,
                order: Ascendant,
            },
        }
    }
    fn prev_self(&mut self) {
        use RuleSortBy::*;
        use SortOrder::*;

        *self = match (self.by, self.order) {
            (RuleName, Ascendant) => Self {
                by: Noop,
                order: Ascendant,
            },
            (RuleName, Descendant) => Self {
                by: RuleName,
                order: Ascendant,
            },
            (Type, Ascendant) => Self {
                by: RuleName,
                order: Descendant,
            },
            (Type, Descendant) => Self {
                by: Type,
                order: Ascendant,
            },
            (ProxyName, Ascendant) => Self {
                by: Type,
                order: Descendant,
            },
            (ProxyName, Descendant) => Self {
                by: ProxyName,
                order: Ascendant,
            },
            (Noop, _) => Self {
                by: ProxyName,
                order: Descendant,
            },
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
        todo!()
    }
}

impl Default for RuleSort {
    fn default() -> Self {
        Self::noop()
    }
}
