use tui::widgets::Widget;

use crate::define_widget;

define_widget!(ConfigPage);

impl<'a> Widget for ConfigPage<'a> {
    fn render(self, _area: tui::layout::Rect, _buf: &mut tui::buffer::Buffer) {}
}
