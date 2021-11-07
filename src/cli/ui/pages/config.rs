use tui::widgets::StatefulWidget;

use crate::cli::EventHandler;

#[derive(Clone, Debug, Default)]
pub struct ConfigPage {}

impl StatefulWidget for ConfigPage {
    type State = ConfigState;
    fn render(
        self,
        area: tui::layout::Rect,
        buf: &mut tui::buffer::Buffer,
        state: &mut Self::State,
    ) {
    }
}

#[derive(Clone, Debug, Default)]
pub struct ConfigState {}

impl EventHandler for ConfigState {
    fn handle(&mut self, event: &crate::cli::Event) -> crate::Result<()> {
        Ok(())
    }
}
