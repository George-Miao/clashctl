use std::marker::PhantomData;

use hhmmss::Hhmmss;
use tui::layout::{Constraint, Layout};
use tui::widgets::{Paragraph, StatefulWidget, Widget};

use crate::cli::{
    components::{get_block, get_text_style, GenericStatefulWidget, MovableList},
    TuiStates,
};

#[derive(Clone, Debug, Default)]
pub struct DebugPage<'a> {
    _life: PhantomData<&'a ()>,
}

impl<'a> StatefulWidget for DebugPage<'a> {
    type State = TuiStates<'a>;
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

        let offset = &mut state.debug_state.offset;

        let debug_info = [
            ("Event In Mem:", event_num.to_string()),
            ("Event All #:", state.all_events_recv.to_string()),
            ("Tick #:", state.ticks.to_string()),
            ("Logs #:", state.log_state.len().to_string()),
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

        let events = MovableList::new("Events");

        info.render(layout[0], buf);
        GenericStatefulWidget::<String>::render(events, layout[1], buf, &mut state.debug_state);
    }
}
