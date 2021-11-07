use tui::widgets::{Block, BorderType, Borders, Paragraph, StatefulWidget, Widget};

use crate::cli::{Event, EventHandler};

#[derive(Clone, Debug, Default)]
pub struct ProxiesPage {}

impl StatefulWidget for ProxiesPage {
    type State = ProxiesState;
    fn render(
        self,
        area: tui::layout::Rect,
        buf: &mut tui::buffer::Buffer,
        state: &mut Self::State,
    ) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title("Proxies");
        let text = Paragraph::new(format!("Event numbers: {}", state.events)).block(block);
        text.render(area, buf)
    }
}

#[derive(Clone, Debug, Default)]
pub struct ProxiesState {
    events: u64,
}

impl EventHandler for ProxiesState {
    fn handle(&mut self, event: &crate::cli::Event) -> crate::Result<()> {
        match event {
            Event::Update => self.events += 1,
            Event::Log(_log) => {}
            _ => {}
        }
        Ok(())
    }
}
