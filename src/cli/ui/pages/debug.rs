use hhmmss::Hhmmss;
use tui::layout::{Constraint, Layout};
use tui::widgets::{Paragraph, StatefulWidget, Widget};

use crate::cli::{
    components::{get_block, get_text_style, MovableList, MovableListState},
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
        let elapsed = state.start_time.map(|x| x.elapsed().as_secs_f64());

        let event_rate = if let Some(elapsed) = elapsed {
            format!("{:.2}/s", event_num as f64 / elapsed)
        } else {
            "?".to_owned()
        };
        let tick_rate = if let Some(elapsed) = elapsed {
            format!("{:.2}/s", state.ticks as f64 / elapsed)
        } else {
            "?".to_owned()
        };

        let offset = state.debug_list_offset;

        let debug_info = [
            ("Event In Mem:", event_num.to_string()),
            ("Event All #:", state.all_events_recv.to_string()),
            ("Event rate:", event_rate),
            ("Tick #:", state.ticks.to_string()),
            ("Tick rate:", tick_rate),
            (
                "List offset: ",
                if offset.hold {
                    format!("({}, {})", offset.x, offset.y)
                } else {
                    "?".to_owned()
                },
            ),
            (
                "Run time:",
                state
                    .start_time
                    .map(|x| x.elapsed().hhmmss())
                    .unwrap_or_else(|| "?".to_owned()),
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

        let items = state
            .events
            .iter()
            .map(|x| format!("{:?}", x))
            .collect::<Vec<_>>();

        let events = MovableList::new("Events");
        let mut list_state = MovableListState {
            items,
            offset: state.debug_list_offset,
        };

        info.render(layout[0], buf);
        StatefulWidget::render(events, layout[1], buf, &mut list_state);
        state.debug_list_offset = list_state.offset;
    }
}
