use std::marker::PhantomData;

use tui::widgets::List;
use tui::{
    text::Spans,
    widgets::{ListItem, Widget},
};
use unicode_width::UnicodeWidthStr;

use crate::cli::{
    components::{
        get_block, get_focused_block, get_text_style, spans_window, GenericStatefulWidget,
    },
    Coord,
};

#[derive(Clone, Debug)]
pub struct MovableList<'a, T> {
    title: String,
    _life: PhantomData<&'a T>,
}

impl<'a, T> MovableList<'a, T>
where
    T: Into<Spans<'a>>,
{
    pub fn new<TITLE: Into<String>>(title: TITLE) -> Self {
        Self {
            title: title.into(),
            _life: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct MovableListState<'a, T>
where
    T: Into<Spans<'a>>,
{
    pub offset: &'a mut Coord,
    pub items: Vec<T>,
    _life: PhantomData<&'a T>,
}

impl<'a, T> MovableListState<'a, T>
where
    T: Into<Spans<'a>>,
{
    pub fn new(items: Vec<T>, offset: &'a mut Coord) -> Self {
        Self {
            offset,
            items,
            _life: PhantomData,
        }
    }
}

impl<'a> GenericStatefulWidget<String> for MovableList<'a, String> {
    type State = MovableListState<'a, String>;
    fn render(
        self,
        area: tui::layout::Rect,
        buf: &mut tui::buffer::Buffer,
        state: &mut Self::State,
    ) {
        let height = (area.height as usize).saturating_sub(2);
        let num = state.items.len();

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
            .take(area.height as usize)
            .clone();

        // Calculate what is current x offset
        // A.K.A. which part will be hidden
        let x_offset = state.offset.x.min(
            items
                .clone()
                .map(|x| x.width())
                .min()
                .unwrap_or_default()
                .saturating_sub(1),
        );

        let items = items.map(|x| -> Spans { x.split_at(x_offset).1.into() });

        // Limit how many chars will be hidden overall
        // Apply offsets back so the offset is being limited to current one
        // Even for next tick
        state.offset.x = x_offset;
        state.offset.y = y_offset;

        let block = if state.offset.hold {
            get_focused_block(&self.title)
        } else {
            get_block(&self.title)
        };

        // Spans window is expensive
        // However it is needed to display part of spans while keeping its style
        let items = List::new(items.map(ListItem::new).collect::<Vec<_>>())
            .block(block)
            .style(get_text_style());

        Widget::render(items, area, buf)
    }
}

impl<'a> GenericStatefulWidget<Spans<'a>> for MovableList<'a, Spans<'a>> {
    type State = MovableListState<'a, Spans<'a>>;
    fn render(
        self,
        area: tui::layout::Rect,
        buf: &mut tui::buffer::Buffer,
        state: &mut Self::State,
    ) {
        let height = (area.height as usize).saturating_sub(2);
        let num = state.items.len();

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
            .take(area.height as usize)
            .to_owned();

        // Calculate what is current x offset
        // A.K.A. which part will be hidden
        let x_offset = state.offset.x.min(
            items
                .clone()
                .map(|x| x.width())
                .min()
                .unwrap_or_default()
                .saturating_sub(1),
        );

        // Limit how many chars will be hidden overall
        // Apply offsets back so the offset is being limited to current one
        // Even for next tick
        state.offset.x = x_offset;
        state.offset.y = y_offset;

        let block = if state.offset.hold {
            get_focused_block(&self.title)
        } else {
            get_block(&self.title)
        };

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

        Widget::render(items, area, buf)
    }
}
