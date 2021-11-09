use tui::widgets::{StatefulWidget, Widget};

use crate::cli::{components::get_block, TuiStates};

#[derive(Clone, Debug, Default)]
pub struct ProxiesPage {}

impl StatefulWidget for ProxiesPage {
    type State = TuiStates;
    fn render(
        self,
        area: tui::layout::Rect,
        buf: &mut tui::buffer::Buffer,
        state: &mut Self::State,
    ) {
        let block = get_block("Proxies");
        block.render(area, buf)
    }
}
