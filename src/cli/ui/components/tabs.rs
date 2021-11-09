use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{StatefulWidget, Tabs as TuiTabs, Widget};

use crate::cli::{components::get_block, Event, EventHandler, InterfaceEvent};

#[derive(Clone, Debug)]
pub struct TabState {
    pub index: usize,
}

#[allow(clippy::derivable_impls)]
impl Default for TabState {
    fn default() -> Self {
        Self { index: 0 }
    }
}

impl TabState {
    pub const TITLES: [&'static str; 4] = ["Status", "Proxies", "Configs", "Debug"];

    pub fn title(&self) -> &str {
        Self::TITLES[self.index]
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % Self::TITLES.len();
    }

    pub fn prev(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = Self::TITLES.len() - 1;
        }
    }
}

impl EventHandler for TabState {
    fn handle(&mut self, event: &Event) -> crate::Result<()> {
        match event {
            Event::Interface(InterfaceEvent::TabNext) => self.next(),
            Event::Interface(InterfaceEvent::TabPrev) => self.prev(),
            Event::Interface(InterfaceEvent::TabGoto(index)) => {
                if index < &Self::TITLES.len() {
                    self.index = *index
                }
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Default, Clone, Debug)]
pub struct Tabs {}

impl StatefulWidget for Tabs {
    type State = TabState;
    fn render(
        self,
        area: tui::layout::Rect,
        buf: &mut tui::buffer::Buffer,
        state: &mut Self::State,
    ) {
        let titles = TabState::TITLES
            .iter()
            .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::DarkGray))))
            .collect();
        let tabs = TuiTabs::new(titles)
            .block(get_block("Clashctl"))
            .highlight_style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )
            .select(state.index);
        tabs.render(area, buf)
    }
}
