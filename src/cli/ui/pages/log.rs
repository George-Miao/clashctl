use std::marker::PhantomData;

use tui::{text::Spans, widgets::Widget};

use crate::{
    cli::components::{GenericWidget, MovableList},
    define_widget,
};

define_widget!(LogPage);

impl<'a> Widget for LogPage<'a> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let list = MovableList::new("Logs", &self.state.log_state);
        GenericWidget::<Spans>::render(list, area, buf);
    }
}
