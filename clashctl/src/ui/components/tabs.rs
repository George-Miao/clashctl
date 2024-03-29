use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Tabs as TuiTabs, Widget},
};

use crate::ui::{define_widget, utils::get_block, TuiStates};

define_widget!(Tabs);

impl<'a> Widget for Tabs<'a> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let len = TuiStates::TITLES.len();
        let range = if self.state.show_debug {
            0..len
        } else {
            0..len - 1
        };
        let titles = TuiStates::TITLES[range]
            .iter()
            .enumerate()
            .map(|(i, t)| {
                Spans::from(Span::styled(
                    format!("{} {}", i + 1, t),
                    Style::default().fg(Color::DarkGray),
                ))
            })
            .collect();
        let tabs = TuiTabs::new(titles)
            .block(get_block("Clashctl"))
            .highlight_style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )
            .select(self.state.page_index.into());
        tabs.render(area, buf)
    }
}
