use tui::widgets::{List, ListItem, StatefulWidget, Widget};

use crate::cli::{
    components::{get_block, get_text_style},
    TuiStates,
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
        let block = get_block("Logs");

        let list = List::new(
            state
                .logs
                .iter()
                .rev()
                .take(block.inner(area).height as usize)
                .map(|x| ListItem::new(format!("{:?}", x)))
                .collect::<Vec<_>>(),
        )
        .block(block)
        .style(get_text_style());

        Widget::render(list, area, buf);
    }
}
