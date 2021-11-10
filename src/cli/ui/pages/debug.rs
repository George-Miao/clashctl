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
            .constraints([Constraint::Length(30), Constraint::Min(0)])
            .direction(tui::layout::Direction::Horizontal)
            .split(area);

        let event_num = state.events.len();
        let elapsed = state.start_time.elapsed().as_secs_f64();

        let debug_info = [
            ("Event #:", event_num.to_string()),
            (
                "Event rate:",
                format!("{:.2}/s", event_num as f64 / elapsed),
            ),
            ("Tick #:", state.ticks.to_string()),
            (
                "Tick rate:",
                format!("{:.2}/s", state.ticks as f64 / elapsed),
            ),
        ]
        .into_iter()
        .map(|(title, content)| format!(" {:<15}{:>11} ", title, content))
        .fold(String::with_capacity(255), |mut a, b| {
            a.push_str(&b);
            a.push('\n');
            a
        });

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
