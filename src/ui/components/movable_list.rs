use std::marker::PhantomData;

use tui::{
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, List},
};
use tui::{
    text::Spans,
    widgets::{ListItem, Widget},
};
use unicode_width::UnicodeWidthStr;

use crate::ui::{
    components::{
        get_block, get_focused_block, get_substring, get_text_style, spans_window, GenericWidget,
    },
    Coord,
};

/// TODO Change [`GenericWidget`] into `MovableListItem` or same thing
///
/// E.g.
///   
/// ```rust
/// pub enum MovableListItem<'a> {
///     Spans(Spans<'a>),
///     Raw(Cow<'a, str>)
/// }
///
/// impl MovableListItem {
///     pub fn new() { todo!() }
///
///     pub fn width(&self) {
///         match self {
///             MovableListItem::Spans(x) => x.width(),
///             MovableListItem::Raw(x) => x.width()
///         }
///     }
///
///     pub fn scope(&self, range: Range) -> Self {
///         match self {
///             MovableListItem::Spans(x) => Self::Spans(spans_window(x, range)),
///             MovableListItem::Raw(x) => Self::raw(x[range].into())
///         }
///     }
///
///     pub fn render() { todo!() } // Maybe render here
/// }
/// ```
#[derive(Clone, Debug)]
pub struct MovableList<'a, T> {
    title: String,
    state: &'a MovableListState<'a, T>,
    _life: PhantomData<&'a T>,
}

impl<'a, T> MovableList<'a, T>
where
    T: Into<Spans<'a>>,
{
    pub fn new<TITLE: Into<String>>(title: TITLE, state: &'a MovableListState<'a, T>) -> Self {
        Self {
            state,
            title: title.into(),
            _life: PhantomData,
        }
    }

    fn render_index(&self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer, pos: Coord) {
        let index_content = format!(" Ln {}, Col {} ", pos.y, pos.x);
        let width = index_content.len();
        let index = Span::styled(
            index_content,
            Style::default()
                .fg(if pos.hold { Color::Green } else { Color::Blue })
                .add_modifier(Modifier::REVERSED),
        );

        if pos.hold {
            let help = Span::styled(
                " [^] ▲ ▼ ◀ ▶ Move ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::REVERSED),
            );
            let mode = Span::styled(
                " HOLD ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::REVERSED),
            );
            buf.set_span(
                area.x + 2,
                area.y + area.height - 1,
                &help,
                help.width() as u16,
            );
            buf.set_span(
                area.x
                    + (area.width as u16).saturating_sub(width.try_into().unwrap_or(u16::MAX) + 9),
                area.y + area.height - 1,
                &mode,
                width.try_into().unwrap_or(u16::MAX),
            );
        }

        buf.set_span(
            area.x + (area.width as u16).saturating_sub(width.try_into().unwrap_or(u16::MAX) + 2),
            area.y + area.height - 1,
            &index,
            width.try_into().unwrap_or(u16::MAX),
        );
    }

    fn prepare<'b, F>(
        &self,
        area: &tui::layout::Rect,
        state: &'b MovableListState<T>,
        width_fn: F,
    ) -> (impl Iterator<Item = &'b T>, Block, usize, usize)
    where
        F: Fn(&T) -> usize,
    {
        let height = (area.height as usize).saturating_sub(2);
        let num = state.items.len();

        let block = if state.offset.hold {
            get_focused_block(&self.title)
        } else {
            get_block(&self.title)
        };

        // Calculate which portion of the list will be displayed
        let y_offset = if height + state.offset.y > num {
            num.saturating_sub(height)
        } else {
            state.offset.y
        };

        // Get that portion of items
        let items = state
            .items
            .iter()
            .rev()
            .skip(y_offset)
            .take(area.height as usize);
        // Calculate what is current x offset
        // A.K.A. which part will be hidden
        let x_offset = state.offset.x.min(
            items
                .clone()
                .map(width_fn)
                .min()
                .unwrap_or_default()
                .saturating_sub(1),
        );

        // Limit how many chars will be hidden overall
        // Apply offsets back so the offset is being limited to current one
        // Even for next tick

        (items, block, x_offset, y_offset)
    }
}

#[derive(Debug, Default, Clone)]
pub struct MovableListState<'a, T> {
    pub offset: Coord,
    pub items: Vec<T>,
    _life: PhantomData<&'a T>,
}

impl<'a, T> MovableListState<'a, T> {
    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl<'a> GenericWidget<String> for MovableList<'a, String> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let (items, block, x_offset, y_offset) = self.prepare(&area, self.state, |x| x.width());

        let list = List::new(
            items
                .map(|x| -> Spans { get_substring(x, x_offset).unwrap().into() })
                .map(ListItem::new)
                .collect::<Vec<_>>(),
        )
        .block(block)
        .style(get_text_style());

        Widget::render(list, area, buf);
        let pos = Coord {
            x: x_offset,
            y: (self.state.items.len() - y_offset + 2).saturating_sub(area.height as usize),
            hold: self.state.offset.hold,
        };
        self.render_index(area, buf, pos)
    }
}

impl<'a> GenericWidget<Spans<'a>> for MovableList<'a, Spans<'a>> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer)
    where
        Self: 'a,
    {
        let (items, block, x_offset, y_offset) = self.prepare(&area, self.state, |x| x.width());
        // Spans window is expensive
        // However it is needed to display part of spans while keeping its style
        let items = List::new(
            items
                .map(|x| {
                    ListItem::new(spans_window(
                        x.to_owned(),
                        x_offset..x_offset + area.width as usize,
                    ))
                })
                .collect::<Vec<_>>(),
        )
        .block(block)
        .style(get_text_style());

        Widget::render(items, area, buf);
        let pos = Coord {
            x: x_offset,
            y: (self.state.items.len() - y_offset + 2).saturating_sub(area.height as usize),
            hold: self.state.offset.hold,
        };
        self.render_index(area, buf, pos)
    }
}
