use tui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{List, ListItem, Widget},
};

use crate::{get_block, ConfigState};

#[derive(Clone, Debug)]
pub struct ConfigPage<'a> {
    state: &'a ConfigState,
}

impl<'a> ConfigPage<'a> {
    pub fn new(state: &'a ConfigState) -> Self {
        Self { state }
    }
}

enum ConfigListItem<'a> {
    Title(&'a str),
    Item { label: &'a str, content: String },
    Separator,
    Empty,
}

impl<'a> ConfigListItem<'a> {
    pub fn title(title: &'a str) -> impl Iterator<Item = ConfigListItem> {
        [
            ConfigListItem::Empty,
            ConfigListItem::Title(title),
            ConfigListItem::Separator,
        ]
        .into_iter()
    }

    pub fn into_list_item(self, width: u16) -> ListItem<'a> {
        match self {
            ConfigListItem::Title(title) => ListItem::new(title).style(
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            ConfigListItem::Item { label, content } => ListItem::new(format!(
                "{:<15}{:>right$}",
                label,
                content,
                right = (width - 15) as usize
            ))
            .style(Style::default().fg(Color::White)),
            ConfigListItem::Separator => {
                ListItem::new(format!("{:-<width$}", "", width = width as usize))
            }
            ConfigListItem::Empty => {
                ListItem::new(format!("{:width$}", "", width = width as usize))
            }
        }
    }
}

impl<'a> Widget for ConfigPage<'a> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let width = area.width.saturating_sub(4).max(10);
        let block = get_block("Config");
        let list = ConfigListItem::title("Clash")
            .chain(self.state.clash_list().map(|x| ConfigListItem::Item {
                label: x.0,
                content: x.1,
            }))
            .chain(ConfigListItem::title("Clashctl"))
            .chain(self.state.clashctl_list().map(|x| ConfigListItem::Item {
                label: x.0,
                content: x.1,
            }))
            .map(|x| x.into_list_item(width))
            .collect::<Vec<_>>();
        let inner = block.inner(area);
        let inner = Rect {
            x: inner.x + 1,
            width: inner.width - 1,
            ..inner
        };
        block.render(area, buf);
        List::new(list).render(inner, buf);
    }
}
