use tui::layout::{Constraint, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, StatefulWidget, Widget};

use crate::cli::{Event, EventHandler, Ring};
use crate::model::Traffic;

#[derive(Default, Clone, Debug)]
pub struct TrafficState {
    pub traffics: Ring<Traffic, 100>,
}

impl TrafficState {}

impl EventHandler for TrafficState {
    fn handle(&mut self, event: &Event) -> crate::Result<()> {
        if let Event::Traffic(traffic) = event {
            self.traffics.push(traffic.to_owned());
        }
        Ok(())
    }
}

#[derive(Default, Clone, Debug)]
pub struct Traffics {}

impl StatefulWidget for Traffics {
    type State = TrafficState;
    fn render(
        self,
        area: tui::layout::Rect,
        buf: &mut tui::buffer::Buffer,
        state: &mut Self::State,
    ) {
        let half = Constraint::Percentage(50);
        let layout = Layout::default()
            .direction(tui::layout::Direction::Vertical)
            .constraints([half, half]);
        let iter = state.traffics.iter();
        let up = iter.map(|x| x.up).take(50);
    }
}
