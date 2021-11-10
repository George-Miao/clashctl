use tui::{
    layout::Rect,
    style::Style,
    text::{Span, Spans, Text},
    widgets::{List, ListItem, StatefulWidget, Widget},
};

use crate::{
    cli::{
        components::{get_block, get_focused_block, get_text_style},
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
        let block = if state.focus {
            get_focused_block("Logs")
        } else {
            get_block("Logs")
        };

        let to_spans = |val: Log| -> Text {
            let color = val.log_type.clone().into();
            Spans::from(vec![
                Span::styled(
                    val.log_type.to_string().to_uppercase(),
                    Style::default().fg(color),
                ),
                Span::raw(" "),
                Span::raw(val.payload),
            ])
            .into()
        };

        let list = List::new(
            state
                .logs
                .iter()
                .rev()
                .take(block.inner(area).height as usize)
                .cloned()
                .map(|x| ListItem::new(to_spans(x)))
                .collect::<Vec<_>>(),
        )
        .style(get_text_style());

        let inner = block.inner(area);

        block.render(area, buf);
        Widget::render(list, inner, buf);
    }
}
