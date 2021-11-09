use tui::layout::{Constraint, Layout};
use tui::{
    text::Spans,
    widgets::{List, ListItem, Paragraph, StatefulWidget, Widget},
};

use crate::cli::{
    components::{get_block, get_text_style},
    TuiStates,
};

#[derive(Clone, Debug, Default)]
pub struct DebugPage {}

impl StatefulWidget for DebugPage {
    type State = TuiStates;
    fn render(
        self,
        area: tui::layout::Rect,
        buf: &mut tui::buffer::Buffer,
        state: &mut Self::State,
    ) {
        let layout = Layout::default()
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
            .direction(tui::layout::Direction::Horizontal)
            .split(area);

        let debug_info = vec![
            format!("Event #: {}", state.events.len()),
            format!("Tick #: {}", state.ticks),
            format!(
                "Tick rate: {}",
                (state.ticks as f64 / state.start_time.elapsed().as_secs_f64()) as usize
            ),
        ]
        .into_iter()
        .map(Spans::from)
        .collect::<Vec<_>>();

        let info = Paragraph::new(debug_info)
            .block(get_block("Debug Info"))
            .style(get_text_style());

        let events = List::new(
            state
                .events
                .iter()
                .rev()
                .take(layout[1].height as usize)
                .map(|x| ListItem::new(format!("{:?}", x)))
                .collect::<Vec<_>>(),
        )
        .block(get_block("Events"))
        .style(get_text_style());

        info.render(layout[0], buf);
        Widget::render(events, layout[1], buf)
    }
}
