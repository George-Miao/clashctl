use std::marker::PhantomData;

use tui::widgets::{StatefulWidget, Widget};

use crate::cli::{components::get_block, TuiStates};

#[derive(Clone, Debug, Default)]
pub struct ProxiesPage<'a> {
    _life: PhantomData<&'a ()>,
}

impl<'a> StatefulWidget for ProxiesPage<'a> {
    type State = TuiStates<'a>;
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
