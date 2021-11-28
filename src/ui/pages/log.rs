use tui::{
    style::Style,
    text::{Span, Spans},
    widgets::Widget,
};

use crate::{
    define_widget,
    model::Log,
    ui::{
        components::{MovableList, MovableListItem},
        AsColor,
    },
};

impl<'a> From<Log> for Spans<'a> {
    fn from(val: Log) -> Self {
        let color = val.log_type.as_color();
        Spans::from(vec![
            Span::styled(
                format!("{:<5}", val.log_type.to_string().to_uppercase()),
                Style::default().fg(color),
            ),
            Span::raw(" "),
            Span::raw(val.payload),
        ])
    }
}

impl<'a> MovableListItem<'a> for Log {
    fn to_spans(&self) -> Spans<'a> {
        let color = self.log_type.clone().as_color();
        Spans::from(vec![
            Span::styled(
                format!("{:<5}", self.log_type.to_string().to_uppercase()),
                Style::default().fg(color),
            ),
            Span::raw(" "),
            Span::raw(self.payload.to_owned()),
        ])
    }
}

define_widget!(LogPage);

// TODO Pretty print parsed Log
impl<'a> Widget for LogPage<'a> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let list = MovableList::new("Logs", &self.state.log_state);
        list.render(area, buf);
    }
}
