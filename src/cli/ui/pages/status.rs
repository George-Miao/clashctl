use bytesize::ByteSize;
use tui::{
    layout::{Constraint, Direction, Layout},
    text::Spans,
    widgets::{Paragraph, StatefulWidget, Widget},
};

use crate::cli::{
    components::{get_block, get_text_style},
    EventHandler,
};
use crate::{
    cli::components::{TrafficState, Traffics},
    model::Traffic,
};

#[derive(Clone, Debug, Default)]
pub struct StatusPage {}

impl StatefulWidget for StatusPage {
    type State = StatusState;
    fn render(
        self,
        area: tui::layout::Rect,
        buf: &mut tui::buffer::Buffer,
        state: &mut Self::State,
    ) {
        let main = Layout::default()
            .constraints([Constraint::Length(30), Constraint::Min(0)])
            .direction(Direction::Horizontal)
            .split(area);

        let last_traffic = state
            .traffic
            .traffics
            .iter()
            .last()
            .map(|x| x.to_owned())
            .unwrap_or_else(Traffic::default);

        let info = [
            ("▲", ByteSize(last_traffic.up).to_string_as(true) + "/s"),
            ("▼", ByteSize(last_traffic.down).to_string_as(true) + "/s"),
            ("▲ Total", "?".to_owned()),
            ("▼ Total", "?".to_owned()),
            ("Connection #", "?".to_owned()),
        ]
        .into_iter()
        .map(|(title, content)| Spans::from(format!(" {:<15}{:>11} ", title, content)))
        .collect::<Vec<_>>();

        Paragraph::new(info)
            .block(get_block("Info"))
            .style(get_text_style())
            .render(main[0], buf);

        let traffic = Traffics::default();
        traffic.render(main[1], buf, &mut state.traffic)
    }
}

#[derive(Clone, Debug, Default)]
pub struct StatusState {
    traffic: TrafficState,
}

impl EventHandler for StatusState {
    fn handle(&mut self, event: &crate::cli::Event) -> crate::Result<()> {
        self.traffic.handle(event)?;
        Ok(())
    }
}
