use tui::widgets::Widget;

use crate::define_widget;

define_widget!(RulesPage);

impl<'a> Widget for RulesPage<'a> {
    fn render(self, _area: tui::layout::Rect, _buf: &mut tui::buffer::Buffer) {}
}
