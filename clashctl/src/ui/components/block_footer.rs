use std::marker::PhantomData;

use tui::{
    layout::Rect,
    text::{Span, Spans},
    widgets::Widget,
};

use crate::ui::Wrap;

#[derive(Debug, Clone, PartialEq)]
pub struct Footer<'a> {
    left_offset: u16,
    right_offset: u16,
    left: Vec<FooterItem<'a>>,
    right: Vec<FooterItem<'a>>,
    _life: PhantomData<&'a ()>,
}

impl<'a> Default for Footer<'a> {
    fn default() -> Self {
        Self {
            left_offset: 2,
            right_offset: 2,
            left: Default::default(),
            right: Default::default(),
            _life: Default::default(),
        }
    }
}

impl<'a> Footer<'a> {
    pub fn show(&mut self) {
        self.items_mut().for_each(FooterItem::show);
    }

    pub fn hide(&mut self) {
        self.items_mut().for_each(FooterItem::hide);
    }

    pub fn items(&self) -> impl Iterator<Item = &FooterItem<'_>> {
        self.left.iter().chain(self.right.iter())
    }

    pub fn items_mut(&mut self) -> impl Iterator<Item = &mut FooterItem<'a>> {
        self.left.iter_mut().chain(self.right.iter_mut())
    }

    pub fn left_offset(&mut self, offset: u16) -> &mut Self {
        self.left_offset = offset;
        self
    }

    pub fn right_offset(&mut self, offset: u16) -> &mut Self {
        self.right_offset = offset;
        self
    }

    pub fn push_left(&mut self, item: FooterItem<'a>) -> &mut Self {
        self.left.push(item);
        self
    }

    pub fn push_right(&mut self, item: FooterItem<'a>) -> &mut Self {
        self.right.push(item);
        self
    }

    pub fn append_left(&mut self, item: &mut Vec<FooterItem<'a>>) -> &mut Self {
        self.left.append(item);
        self
    }

    pub fn append_right(&mut self, item: &mut Vec<FooterItem<'a>>) -> &mut Self {
        self.right.append(item);
        self
    }

    pub fn pop_left(&mut self) -> Option<FooterItem<'a>> {
        self.left.pop()
    }

    pub fn pop_right(&mut self) -> Option<FooterItem<'a>> {
        self.right.pop()
    }
}

#[derive(Debug, Clone)]
pub struct FooterWidget<'a> {
    state: &'a Footer<'a>,
}

impl<'a> FooterWidget<'a> {
    pub fn render_one(&mut self, item: Spans, area: Rect, buf: &mut tui::buffer::Buffer) {
        buf.set_spans(area.x, area.y, &item, item.width() as u16);
    }

    pub fn new(state: &'a Footer) -> Self {
        Self { state }
    }
}

impl<'a> Widget for FooterWidget<'a> {
    fn render(self, area: Rect, buf: &mut tui::buffer::Buffer) {
        let y = area.y + area.height - 1;
        let (mut left, mut right) = (self.state.left.iter(), self.state.right.iter());
        let (mut left_x, mut right_x) = (
            area.x.saturating_add(self.state.left_offset),
            area.x
                .saturating_add(area.width + 1)
                .saturating_sub(self.state.right_offset),
        );
        loop {
            let mut changed = false;
            if let Some(spans) = left.next() {
                if spans.show {
                    let spans = spans.to_spans();
                    let width = spans.width() as u16;
                    if right_x.saturating_sub(left_x) <= width {
                        break;
                    }
                    buf.set_spans(left_x, y, &spans, width);
                    left_x += width + 1;

                    changed = true;
                }
            }

            if let Some(spans) = right.next() {
                if spans.show {
                    let spans = spans.to_spans();
                    let width = spans.width() as u16;
                    if right_x.saturating_sub(left_x) <= width {
                        break;
                    }
                    right_x = right_x.saturating_sub(width + 1);
                    buf.set_spans(right_x, y, &spans, width);
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FooterItem<'a> {
    inner: FooterItemInner<'a>,
    show: bool,
}

impl<'a> FooterItem<'a> {
    pub fn to_spans(&self) -> Spans<'a> {
        match self.inner {
            FooterItemInner::Raw(ref raw) => Spans::from(raw.to_string()),
            FooterItemInner::Span(ref span) => span.to_owned().into(),
            FooterItemInner::Spans(ref spans) => spans.to_owned(),
        }
    }

    pub fn raw(content: String) -> Self {
        Self {
            inner: FooterItemInner::Raw(content),
            show: true,
        }
    }

    pub fn span(content: Span<'a>) -> Self {
        Self {
            inner: FooterItemInner::Span(content),
            show: true,
        }
    }

    pub fn spans(content: Spans<'a>) -> Self {
        Self {
            inner: FooterItemInner::Spans(content),
            show: true,
        }
    }

    pub fn set_show(&mut self, show: bool) {
        self.show = show
    }

    pub fn show(&mut self) {
        self.set_show(true)
    }

    pub fn hide(&mut self) {
        self.set_show(false)
    }
}

impl<'a> From<Span<'a>> for FooterItem<'a> {
    fn from(val: Span<'a>) -> Self {
        Self::span(val)
    }
}

impl<'a> From<Spans<'a>> for FooterItem<'a> {
    fn from(val: Spans<'a>) -> Self {
        Self::spans(val)
    }
}

impl<'a> From<String> for FooterItem<'a> {
    fn from(val: String) -> Self {
        Self::raw(val)
    }
}

#[derive(Debug, Clone, PartialEq)]
enum FooterItemInner<'a> {
    Raw(String),
    Span(Span<'a>),
    Spans(Spans<'a>),
}

impl<'a> From<FooterItemInner<'a>> for Spans<'a> {
    fn from(val: FooterItemInner<'a>) -> Self {
        match val {
            FooterItemInner::Raw(raw) => raw.into(),
            FooterItemInner::Span(span) => span.into(),
            FooterItemInner::Spans(spans) => spans,
        }
    }
}

impl<'a> From<FooterItem<'a>> for Spans<'a> {
    fn from(val: FooterItem<'a>) -> Self {
        val.inner.into()
    }
}

impl<'a> Wrap for FooterItem<'a> {
    fn wrap_by(mut self, char: char) -> Self {
        match self.inner {
            FooterItemInner::Raw(ref mut raw) => {
                raw.wrap_by(char);
            }
            FooterItemInner::Span(ref mut span) => {
                span.wrap_by(char);
            }
            FooterItemInner::Spans(ref mut spans) => {
                spans.wrap_by(char);
            }
        };
        self
    }
}
