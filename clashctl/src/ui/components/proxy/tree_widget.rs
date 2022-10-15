use tui::widgets::{Paragraph, Widget};

use crate::{
    components::{FooterWidget, ProxyGroupFocusStatus, ProxyTree},
    get_block, get_focused_block,
};

#[derive(Clone, Debug)]
pub struct ProxyTreeWidget<'a> {
    state: &'a ProxyTree<'a>,
}

impl<'a> ProxyTreeWidget<'a> {
    pub fn new(state: &'a ProxyTree<'a>) -> Self {
        Self { state }
    }
}

impl<'a> Widget for ProxyTreeWidget<'a> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let cursor = &self.state.cursor;
        let skip = if self.state.expanded {
            *cursor
        } else {
            cursor.saturating_sub(2)
        };
        let text = self
            .state
            .groups
            .iter()
            .skip(skip)
            .enumerate()
            .map(|(i, x)| {
                x.get_widget(
                    area.width as usize,
                    match (self.state.expanded, *cursor == i + skip) {
                        (true, true) => ProxyGroupFocusStatus::Expanded,
                        (false, true) => ProxyGroupFocusStatus::Focused,
                        _ => ProxyGroupFocusStatus::None,
                    },
                )
            })
            .reduce(|mut a, b| {
                a.extend(b);
                a
            })
            .unwrap_or_default()
            .into_iter()
            .take(area.height as usize)
            .collect::<Vec<_>>();

        let block = if self.state.expanded {
            get_focused_block("Proxies")
        } else {
            get_block("Proxies")
        };

        let inner = block.inner(area);

        block.render(area, buf);

        Paragraph::new(text).render(inner, buf);
        FooterWidget::new(&self.state.footer).render(area, buf);
    }
}
