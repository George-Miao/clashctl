use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, StatefulWidget, Tabs as TuiTabs, Widget};

use crate::cli::{Event, EventHandler};

#[derive(Default, Clone, Debug)]
pub struct TabState {
    pub index: usize,
}

impl TabState {
    pub const TITLES: [&'static str; 3] = ["Status", "Proxies", "Configs"];

    pub fn title(&self) -> &str {
        Self::TITLES[self.index]
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % Self::TITLES.len();
    }

    pub fn previous(&mut self) {
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
            Event::TabNext => self.next(),
            Event::TabPrev => self.previous(),
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
            .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::Green))))
            .collect();
        let tabs = TuiTabs::new(titles)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title(clap::crate_name!()),
            )
            .highlight_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
            .select(state.index);
        tabs.render(area, buf)
    }
}
