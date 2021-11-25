use bytesize::ByteSize;
use chrono::Utc;
use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::Widget,
};

use crate::{
    define_widget,
    model::Connections,
    ui::components::{MovableList, MovableListItem, MovableListState},
    ui::HMS,
};

define_widget!(ConnectionsPage);

impl<'a> Widget for ConnectionsPage<'a> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        MovableList::new("Connections", &self.state.con_state).render(area, buf);
    }
}

impl<'a> From<Connections> for MovableListState<'a> {
    fn from(con: Connections) -> Self {
        let dimmed = Style::default().fg(Color::DarkGray);
        // .add_modifier(Modifier::DIM);
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

                let src = format!("{}:{} ", meta.source_ip, meta.source_port);
                let dest = format!(
                    " {}:{}",
                    if meta.destination_ip.is_empty() {
                        "?"
                    } else {
                        &meta.destination_ip
                    },
                    meta.source_port
                );
                let dash: String =
                    "─".repeat(44_usize.saturating_sub(src.len() + dest.len()).max(1));

                let time = (Utc::now() - x.start).hms();
                let spans = vec![
                    Span::styled(format!("{:45}", host), bolded),
                    // Download size
                    Span::styled(" ▼  ", dimmed),
                    Span::raw(format!("{:12}", dl)),
                    // Download speed
                    Span::styled(" ⇊  ", dimmed),
                    Span::raw(format!("{:12}", dl_speed)),
                    // Upload size
                    Span::styled(" ▲  ", dimmed),
                    Span::raw(format!("{:12}", up)),
                    // Upload Speed
                    Span::styled(" ⇈  ", dimmed),
                    Span::raw(format!("{:12}", up_speed)),
                    // Time
                    Span::styled(" ⏲  ", dimmed),
                    Span::raw(format!("{:10}", time)),
                    // Rule
                    Span::styled(" ✤  ", dimmed),
                    Span::raw(format!("{:15}", x.rule)),
                    // IP
                    Span::styled(" ⇄  ", dimmed),
                    Span::raw(src),
                    Span::styled(dash, dimmed),
                    Span::raw(dest),
                    // Chain
                    Span::styled("   ⟴  ", dimmed),
                    Span::raw(x.chains.join(" - ")),
                ];

                MovableListItem::Spans(Spans(spans))
            })
            .collect();
        Self::new(items)
    }
}
