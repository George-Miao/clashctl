use tui::widgets::List;
use tui::widgets::{ListItem, StatefulWidget, Widget};

use crate::cli::{
    components::{get_block, get_focused_block, get_text_style},
    Coord,
};

#[derive(Clone, Debug)]
pub struct MovableList {
    title: String,
}

impl MovableList {
    pub fn new<T: Into<String>>(title: T) -> Self {
        Self {
            title: title.into(),
        }
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
        let height = (area.height as usize).saturating_sub(2);
        let num = state.items.len();

        let y_offset = if height + state.offset.y > num {
            num.saturating_sub(height)
        } else {
            state.offset.y
        };

        let items = state
            .items
            .iter()
            .rev()
            .skip(y_offset)
            .take(area.height as usize);

        let x_offset = state.offset.x.min(
            state
                .items
                .iter()
                .map(|x| x.len().saturating_sub(1))
                .min()
                .unwrap_or_default(),
        );

        state.offset.x = x_offset;
        state.offset.y = y_offset;

        let block = if state.offset.hold {
            get_focused_block("Events")
        } else {
            get_block("Events")
        };

        let items = List::new(
            items
                .map(|x| ListItem::new(x.split_at(x_offset).1.to_owned()))
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
