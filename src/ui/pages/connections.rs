use tui::widgets::Widget;

use crate::define_widget;

define_widget!(ConnectionsPage);

impl<'a> Widget for ConnectionsPage<'a> {
    fn render(self, _area: tui::layout::Rect, _buf: &mut tui::buffer::Buffer) {}
}
