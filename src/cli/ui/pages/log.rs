use tui::{
    style::Style,
    text::{Span, Spans},
    widgets::StatefulWidget,
};

use crate::{
    cli::{
        components::{MovableList, MovableListState},
        TuiStates,
    },
    model::Log,
};

#[derive(Clone, Debug, Default)]
pub struct LogPage {}

impl StatefulWidget for LogPage {
    type State = TuiStates;
    fn render(
        self,
        area: tui::layout::Rect,
        buf: &mut tui::buffer::Buffer,
        state: &mut Self::State,
    ) {
        let to_spans = |val: &Log| -> Spans {
            let color = val.log_type.clone().into();
            Spans::from(vec![
                Span::styled(
                    format!("{:^7}", val.log_type.to_string().to_uppercase()),
                    Style::default().fg(color),
                ),
                Span::raw(" "),
                Span::raw(val.payload.to_owned()),
            ])
        };

        let items = state.logs.iter().map(to_spans).collect::<Vec<_>>();

        let list = MovableList::new(items, "Logs");
        let mut list_state = MovableListState {
            offset: state.log_list_offset,
        };

        StatefulWidget::render(list, area, buf, &mut list_state);
        state.log_list_offset = list_state.offset;
    }
}
