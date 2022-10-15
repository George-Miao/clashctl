use clashctl_core::model::{Rule, RuleType, Rules};
use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::Widget,
};

use crate::{
    components::{MovableList, MovableListItem, MovableListState},
    define_widget,
    interactive::RuleSort,
    AsColor,
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
            RuleType::Unknown => Color::DarkGray,
        }
    }
}

impl<'a> From<Rules> for MovableListState<'a, Rule, RuleSort> {
    fn from(val: Rules) -> Self {
        Self::new_with_sort(val.rules, RuleSort::default())
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
        let payload = if self.payload.is_empty() {
            "*"
        } else {
            &self.payload
        }
        .to_owned();
        let dash: String = "─".repeat(35_usize.saturating_sub(payload.len()) + 2) + " ";
        vec![
            Span::styled(format!("{:16}", r_type), Style::default().fg(type_color)),
            Span::styled(payload + " ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(dash, gray),
            Span::styled(self.proxy.to_owned(), Style::default().fg(name_color)),
        ]
        .into()
    }
}
