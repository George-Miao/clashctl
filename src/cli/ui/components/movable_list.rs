use tui::widgets::List;
use tui::widgets::{ListItem, StatefulWidget, Widget};

use crate::cli::{
    components::{get_block, get_focused_block, get_text_style},
    Coord,
};

#[derive(Clone, Debug)]
pub struct MovableList;

impl MovableList {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Default, Clone, Debug)]
pub struct MovableListState {
    pub items: Vec<String>,
    pub offset: Coord,
}

impl StatefulWidget for MovableList {
    type State = MovableListState;
    fn render(
        self,
        area: tui::layout::Rect,
        buf: &mut tui::buffer::Buffer,
        state: &mut Self::State,
    ) {
        let items = state
            .items
            .iter()
            .rev()
            .skip(state.offset.y)
            .take(area.height as usize);

        let offset = state.offset.x.min(
            state
                .items
                .iter()
                .map(|x| x.len().saturating_sub(1))
                .min()
                .unwrap_or_default(),
        );

        state.offset.x = offset;

        let block = if state.offset.hold {
            get_focused_block("Events")
        } else {
            get_block("Events")
        };

        let items = List::new(
            items
                .map(|x| ListItem::new(x.split_at(offset).1.to_owned()))
                .collect::<Vec<_>>(),
        )
        .block(block)
        .style(get_text_style());

        Widget::render(items, area, buf)
    }
}

impl Widget for MovableList {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let mut state = MovableListState::default();
        StatefulWidget::render(self, area, buf, &mut state)
    }
}
