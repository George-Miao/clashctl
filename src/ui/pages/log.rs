use tui::widgets::Widget;

use crate::{define_widget, ui::components::MovableList};

define_widget!(LogPage);

impl<'a> Widget for LogPage<'a> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let list = MovableList::new("Logs", &self.state.log_state);
        list.render(area, buf);
    }
}
