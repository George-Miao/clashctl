use tui::widgets::StatefulWidget;

use crate::cli::EventHandler;

#[derive(Clone, Debug, Default)]
pub struct StatusPage {}

impl StatefulWidget for StatusPage {
    type State = StatusState;
    fn render(
        self,
        area: tui::layout::Rect,
        buf: &mut tui::buffer::Buffer,
        state: &mut Self::State,
    ) {
    }
}

#[derive(Clone, Debug, Default)]
pub struct StatusState {}

impl EventHandler for StatusState {
    fn handle(&mut self, event: &crate::cli::Event) -> crate::Result<()> {
        Ok(())
    }
}
