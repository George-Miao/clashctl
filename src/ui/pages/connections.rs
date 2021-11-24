use bytesize::ByteSize;
use chrono::Utc;
use hhmmss::Hhmmss;
use tui::{
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::Widget,
};

use crate::{
    components::{MovableList, MovableListItem, MovableListState},
    define_widget,
    model::Connections,
};

define_widget!(ConnectionsPage);

impl<'a> Widget for ConnectionsPage<'a> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        MovableList::new("Connections", &self.state.con_state).render(area, buf);
    }
}

impl<'a> From<Connections> for MovableListState<'a> {
    fn from(con: Connections) -> Self {
        let dimmed = Style::default().add_modifier(Modifier::DIM);
        let bolded = Style::default().add_modifier(Modifier::BOLD);
        let items = con
            .connections
            .into_iter()
            .map(|x| {
                let (dl, up) = (
                    ByteSize(x.download).to_string_as(true),
                    ByteSize(x.upload).to_string_as(true),
                );
                let (dl_speed, up_speed) = (
                    ByteSize(x.down_speed().unwrap_or_default()).to_string_as(true) + "/s",
                    ByteSize(x.up_speed().unwrap_or_default()).to_string_as(true) + "/s",
                );
                let meta = x.metadata;
                let host = format!(" {}:{}", meta.host, meta.destination_port);

                let src = format!("{}:{}", meta.source_ip, meta.source_port);

                let time = (Utc::now() - x.start).hhmmss();
                let spans = vec![
                    Span::styled(format!("{:<45}", host), bolded),
                    // Download size
                    Span::styled("▼  ", dimmed),
                    Span::raw(format!("{:<12}", dl)),
                    // Download speed
                    Span::styled("⇊  ", dimmed),
                    Span::raw(format!("{:<12}", dl_speed)),
                    // Upload size
                    Span::styled("▲  ", dimmed),
                    Span::raw(format!("{:<12}", up)),
                    // Upload Speed
                    Span::styled("⇈  ", dimmed),
                    Span::raw(format!("{:<12}", up_speed)),
                    // Time
                    Span::styled("⏲  ", dimmed),
                    Span::raw(format!("{:<10}", time)),
                    // Chain
                    Span::styled("⛓  ", dimmed),
                    Span::raw(x.chains.join(" - ")),
                ];
                // spans.push(Span::styled(format!("{:<18}", src), dimmed));
                // spans.push(Span::styled("➜  ", dimmed));

                MovableListItem::Spans(Spans(spans))
            })
            .collect();
        Self::new(items)
    }
}
