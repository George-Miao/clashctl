use std::marker::PhantomData;

use tui::widgets::Widget;

use crate::{define_widget, ui::components::ProxyTreeWidget};

define_widget!(ProxiesPage);

impl<'a> Widget for ProxiesPage<'a> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        ProxyTreeWidget::new(&self.state.proxy_tree).render(area, buf);
    }
}
