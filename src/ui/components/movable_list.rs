use std::{borrow::Cow, ops::Range};

use crossterm::event::KeyCode;
use tui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{List, ListItem},
};
use tui::{text::Spans, widgets::Widget};
use unicode_width::UnicodeWidthStr;

use crate::ui::{
    components::{Footer, FooterItem, FooterWidget},
    utils::{get_block, get_focused_block, get_text_style, spans_window, string_window, Coord},
    ListEvent,
};

// TODO Fixed item on top
// Useful for table header
// Append to vec on each render
#[derive(Clone, Debug)]
pub struct MovableList<'a> {
    title: String,
    state: &'a MovableListState<'a>,
}

impl<'a> MovableList<'a> {
    pub fn new<TITLE: Into<String>>(title: TITLE, state: &'a MovableListState<'a>) -> Self {
        Self {
            state,
            title: title.into(),
        }
    }

    fn render_footer(&self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer, pos: Coord) {
        let mut footer = Footer::default();

        footer.push_right(FooterItem::span(Span::styled(
            format!(" Ln {}, Col {} ", pos.y, pos.x),
            Style::default()
                .fg(if pos.hold { Color::Green } else { Color::Blue })
                .add_modifier(Modifier::REVERSED),
        )));

        if pos.hold {
            let style = Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::REVERSED);

            footer.push_left(FooterItem::span(Span::styled(" FREE ", style)));
            footer.push_left(FooterItem::span(Span::styled(" [^] ▲ ▼ ◀ ▶ Move ", style)));
        } else {
            let style = Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::REVERSED);

            footer.push_left(FooterItem::span(Span::styled(" NORMAL ", style)));
            footer.push_left(FooterItem::span(Span::styled(
                " SPACE / [^] ▲ ▼ ◀ ▶ Move ",
                style,
            )));
        }

        let widget = FooterWidget::new(&footer);
        widget.render(area, buf);
    }
}

// TODO: Use lazy updated footer
#[derive(Debug, Clone, PartialEq)]
pub struct MovableListState<'a> {
    offset: Coord,
    items: Vec<MovableListItem<'a>>,
    placeholder: Option<Cow<'a, str>>,
    padding: u16,
}

impl<'a> Default for MovableListState<'a> {
    fn default() -> Self {
        Self::new(vec![])
    }
}

impl<'a> MovableListState<'a> {
    pub fn new(items: Vec<MovableListItem<'a>>) -> Self {
        Self {
            offset: Default::default(),
            items,
            placeholder: None,
            padding: 1,
        }
    }

    pub fn placeholder<T: Into<Cow<'a, str>>>(&mut self, content: T) -> &mut Self {
        self.placeholder = Some(content.into());
        self
    }

    pub fn padding(&mut self, padding: u16) -> &mut Self {
        self.padding = padding;
        self
    }

    pub fn set_items(&mut self, items: Vec<MovableListItem<'a>>) -> &mut Self {
        self.items = items;
        self
    }

    pub fn merge(&mut self, other: Self) {
        if self == &other {
            return;
        }
        self.items = other.items;
    }

    pub fn current_pos(&self) -> Coord {
        let x = self.offset.x;
        let y = self.len().saturating_sub(self.offset.y);
        Coord {
            x,
            y,
            hold: self.offset.hold,
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn toggle(&mut self) {
        self.offset.toggle()
    }

    pub fn end(&mut self) {
        self.offset.end()
    }

    pub fn hold(&mut self) {
        self.offset.hold()
    }

    pub fn push(&mut self, item: MovableListItem<'a>) {
        self.items.push(item);
        if self.offset.hold {
            self.offset.y += 1;
        }
    }

    pub fn handle(&mut self, event: ListEvent) {
        let len = self.len().saturating_sub(1);
        let offset = &mut self.offset;

        if !offset.hold {
            offset.hold = true;
        }

        match (event.fast, event.code) {
            (true, KeyCode::Left) => offset.x = offset.x.saturating_sub(7),
            (true, KeyCode::Right) => offset.x = offset.x.saturating_add(7),
            (true, KeyCode::Up) => offset.y = offset.y.saturating_sub(5),
            (true, KeyCode::Down) => offset.y = offset.y.saturating_add(5).min(len),
            (false, KeyCode::Left) => offset.x = offset.x.saturating_sub(1),
            (false, KeyCode::Right) => offset.x = offset.x.saturating_add(1),
            (false, KeyCode::Up) => offset.y = offset.y.saturating_sub(1),
            (false, KeyCode::Down) => offset.y = offset.y.saturating_add(1).min(len),
            _ => {}
        }
    }

    pub fn offset(&self) -> &Coord {
        &self.offset
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MovableListItem<'a> {
    Spans(Spans<'a>),
    Raw(String),
}

impl<'a> MovableListItem<'a> {
    pub fn width(&self) -> usize {
        match self {
            Self::Spans(x) => x.width(),
            Self::Raw(x) => x.width(),
        }
    }

    pub fn range(&self, range: &Range<usize>) -> MovableListItem {
        match self {
            MovableListItem::Spans(ref x) => MovableListItem::Spans(spans_window(x, range)),
            MovableListItem::Raw(ref x) => MovableListItem::Raw(string_window(x, range)),
        }
    }
}

impl<'a, T: Into<String>> From<T> for MovableListItem<'a> {
    fn from(string: T) -> Self {
        Self::Raw(string.into())
    }
}

impl<'a> From<MovableListItem<'a>> for Spans<'a> {
    fn from(val: MovableListItem<'a>) -> Self {
        match val {
            MovableListItem::Spans(spans) => spans,
            MovableListItem::Raw(raw) => raw.into(),
        }
    }
}

impl<'a> Widget for MovableList<'a> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let num = self.state.items.len();

        let offset = self.state.offset;

        let block = if offset.hold {
            get_focused_block(&self.title)
        } else {
            get_block(&self.title)
        };
        let pad = self.state.padding;
        let inner = block.inner(area);
        let inner = if pad == 0 {
            inner
        } else {
            Rect {
                x: inner.x + pad,
                y: inner.y,
                width: inner.width.saturating_sub(pad * 2),
                height: inner.height,
            }
        };

        let height = inner.height as usize;

        // Calculate which portion of the list will be displayed
        let y_offset = if offset.y + 1 > num {
            num.saturating_sub(1)
        } else {
            offset.y
        };

        let x_offset = offset.x;

        let x_range = x_offset..(x_offset + inner.width as usize);

        // Get that portion of items
        let items = if num != 0 {
            self.state
                .items
                .iter()
                .rev()
                .skip(y_offset)
                .take(height as usize)
                .map(|x| {
                    let x_width = x.width();
                    let mut content = x.range(&x_range);
                    if x_width != 0 && content.width() == 0 {
                        content = MovableListItem::Raw("◀".to_owned());
                    }
                    ListItem::new(Spans::from(content.to_owned()))
                })
                .collect::<Vec<_>>()
        } else {
            vec![ListItem::new(Span::raw(
                self.state
                    .placeholder
                    .to_owned()
                    .unwrap_or_else(|| "Nothing's here yet".into()),
            ))]
        };

        block.render(area, buf);
        List::new(items).style(get_text_style()).render(inner, buf);

        self.render_footer(area, buf, self.state.current_pos());
    }
}

// #[test]
// fn test_movable_list() {
//     let items = &["Test1", "测试1", "[ABCD] 🇺🇲 测试 符号 106"].into_iter().map(|x| x.);
//     assert_eq!()
// }
