use std::marker::PhantomData;

use hhmmss::Hhmmss;
use tui::layout::{Constraint, Layout};
use tui::widgets::{Paragraph, Widget};

use crate::{
    define_widget,
    ui::components::{get_block, get_text_style, GenericWidget, MovableList},
    TICK_COUNTER,
};

define_widget!(DebugPage);

impl<'a> Widget for DebugPage<'a> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let layout = Layout::default()
            .constraints([Constraint::Length(30), Constraint::Min(0)])
            .direction(tui::layout::Direction::Horizontal)
            .split(area);

        let event_num = self.state.events.len();

        let offset = &self.state.debug_state.offset;

        let mut tick = 0;
        let mut tick_rate = None;

        TICK_COUNTER.with(|t| {
            let counter = t.borrow();
            tick = counter.tick_num();
            tick_rate = counter.tick_rate();
        });

        let debug_info = [
            ("Event In Mem:", event_num.to_string()),
            ("Event All #:", self.state.all_events_recv.to_string()),
            ("Tick #:", tick.to_string()),
            (
                "Tick Rate:",
                tick_rate.map_or_else(|| "?".to_owned(), |rate| format!("{:.0}", rate)),
            ),
            ("Logs #:", self.state.log_state.len().to_string()),
            (
                "Proxy group #",
                self.state.proxy_tree.groups.len().to_string(),
            ),
            (
                "List offset: ",
                if offset.hold {
                    format!("({}, {})", offset.x, offset.y)
                } else {
                    "?".to_owned()
                },
            ),
            (
                "Proxy tree:",
                format!(
                    "{} / {}",
                    self.state.proxy_tree.cursor,
                    self.state.proxy_tree.groups.len()
                ),
            ),
            (
                "Run time:",
                self.state
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

        let events = MovableList::new("Events", &self.state.debug_state);

        info.render(layout[0], buf);
        GenericWidget::<String>::render(events, layout[1], buf);
    }
}
