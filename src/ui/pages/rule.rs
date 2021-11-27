use std::collections::HashMap;

use itertools::Itertools;
use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::Widget,
};

use crate::{
    define_widget,
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

impl<'a> From<Rules> for MovableListState<'a> {
    fn from(val: Rules) -> Self {
        let items = val
            .rules
            .into_iter()
            .enumerate()
            .map(|(i, x)| x.into_list_item(i + 1))
            .rev()
            .collect::<Vec<_>>();
        let mut ret = Self::default();
        ret.set_items(items);
        ret
    }
}

impl Rule {
    pub fn into_list_item<'a>(self, index: usize) -> MovableListItem<'a> {
        let type_color = self.rule_type.as_color();
        let name_color = if self.proxy == "DIRECT" || self.proxy == "REJECT" {
            Color::DarkGray
        } else {
            Color::Yellow
        };
        let gray = Style::default().fg(Color::DarkGray);
        let r_type: &'static str = self.rule_type.into();
        let dash: String = "─".repeat(35_usize.saturating_sub(self.payload.len()) + 2) + " ";
        let vec = vec![
            Span::styled(format!("{:>3} ", index), gray),
            Span::styled(format!("{:16}", r_type), Style::default().fg(type_color)),
            Span::styled(
                self.payload + " ",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::styled(dash, gray),
            Span::styled(self.proxy, Style::default().fg(name_color)),
        ];
        MovableListItem::Spans(Spans(vec))
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
}
