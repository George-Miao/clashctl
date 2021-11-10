use tui::widgets::List;
use tui::{
    text::Spans,
    widgets::{ListItem, StatefulWidget, Widget},
};

use crate::cli::{
    components::{get_block, get_focused_block, get_text_style, spans_window},
    Coord,
};

#[derive(Clone, Debug)]
pub struct MovableList<'a> {
    title: String,
    pub items: Vec<Spans<'a>>,
}

impl<'a> MovableList<'a> {
    pub fn new<C: Into<Spans<'a>>, T: Into<String>>(items: Vec<C>, title: T) -> Self {
        Self {
            items: items.into_iter().map(|x| x.into()).collect(),
            title: title.into(),
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct MovableListState {
    pub offset: Coord,
}

impl<'a> StatefulWidget for MovableList<'a> {
    type State = MovableListState;
    fn render(
        self,
        area: tui::layout::Rect,
        buf: &mut tui::buffer::Buffer,
        state: &mut Self::State,
    ) {
        let height = (area.height as usize).saturating_sub(2);
        let num = self.items.len();

        let y_offset = if height + state.offset.y > num {
            num.saturating_sub(height)
        } else {
            state.offset.y
        };

        let x_offset = state.offset.x.min(
            self.items
                .iter()
                .map(|x| x.width())
                .min()
                .unwrap_or_default()
                .saturating_sub(1),
        );

        let items = self
            .items
            .into_iter()
            .rev()
            .skip(y_offset)
            .take(area.height as usize);

        state.offset.x = x_offset;
        state.offset.y = y_offset;

        let block = if state.offset.hold {
            get_focused_block(&self.title)
        } else {
            get_block(&self.title)
        };

        let items = List::new(
            items
                .map(|x| ListItem::new(spans_window(x, x_offset..x_offset + area.width as usize)))
                .collect::<Vec<_>>(),
        )
        .block(block)
        .style(get_text_style());

        Widget::render(items, area, buf)
    }
}

impl<'a> Widget for MovableList<'a> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let mut state = MovableListState::default();
        StatefulWidget::render(self, area, buf, &mut state)
    }
}
