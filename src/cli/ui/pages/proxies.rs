use std::marker::PhantomData;

use tui::widgets::Widget;

use crate::{cli::components::ProxyTreeWidget, define_widget};

define_widget!(ProxiesPage);

impl<'a> Widget for ProxiesPage<'a> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        ProxyTreeWidget::new(&self.state.proxy_tree).render(area, buf);
    }
}
