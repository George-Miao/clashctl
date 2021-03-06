use tui::widgets::Widget;

use crate::{components::ProxyTreeWidget, define_widget};

define_widget!(ProxyPage);

impl<'a> Widget for ProxyPage<'a> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        ProxyTreeWidget::new(&self.state.proxy_tree).render(area, buf);
    }
}
