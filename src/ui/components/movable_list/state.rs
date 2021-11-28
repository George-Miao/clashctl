use std::{borrow::Cow, fmt::Debug};

use crossterm::event::KeyCode;
use paste::paste;
use smart_default::SmartDefault;

use crate::{
    interactive::{EndlessSelf, Noop, SortMethod, Sortable},
    ui::{components::MovableListItem, utils::Coord, ListEvent},
};

macro_rules! impl_setter {
    ($prop:ident, $ty:ty) => {
        paste! {
            pub fn [<set_ $prop>](&mut self, $prop: $ty) -> &mut Self {
                self.$prop = $prop;
                self
            }
        }
    };
    ($prop:ident, $val:expr) => {
        pub fn $prop(&mut self) -> &mut Self {
            self.$prop = $val;
            self
        }
    };
    ($fn_name:ident, $prop:ident, $val:expr) => {
        pub fn $fn_name(&mut self) -> &mut Self {
            self.$prop = $val;
            self
        }
    };
}

// TODO: Use lazy updated footer
#[derive(Debug, Clone, PartialEq, SmartDefault)]
pub struct MovableListState<'a, T: MovableListItem<'a>, S = Noop> {
    pub(super) offset: Coord,
    pub(super) items: Vec<T>,
    pub(super) placeholder: Option<Cow<'a, str>>,
    #[default = 1]
    pub(super) padding: u16,
    pub(super) sort: Option<S>,
    pub(super) with_index: bool,
    pub(super) reverse_index: bool,
}

impl<'a, T, S> MovableListState<'a, T, S>
where
    T: MovableListItem<'a>,
    S: SortMethod<T> + EndlessSelf,
{
    pub fn new(items: Vec<T>) -> Self
    where
        T: MovableListItem<'a>,
    {
        Self {
            items,
            ..Default::default()
        }
    }
    pub fn new_with_sort(mut items: Vec<T>, sort: S) -> Self
    where
        T: MovableListItem<'a>,
    {
        items.sort_by(|a, b| sort.sort_fn(a, b));

        Self {
            items,
            sort: Some(sort),
            ..Default::default()
        }
    }

    impl_setter!(with_index, true);
    impl_setter!(without_index, with_index, false);
    impl_setter!(asc_index, reverse_index, false);
    impl_setter!(dsc_index, reverse_index, true);
    impl_setter!(items, Vec<T>);
    impl_setter!(padding, u16);

    pub fn placeholder<P: Into<Cow<'a, str>>>(&mut self, content: P) -> &mut Self {
        self.placeholder = Some(content.into());
        self
    }
}

impl<'a, T, S> Extend<T> for MovableListState<'a, T, S>
where
    T: MovableListItem<'a>,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.items.extend(iter)
    }
}

impl<'a, T, S> Sortable<'a, S> for MovableListState<'a, T, S>
where
    T: MovableListItem<'a>,
    S: SortMethod<T>,
{
    type Item<'b> = T;
    fn sort_with(&mut self, method: &S) {
        self.items.sort_by(|a, b| method.sort_fn(a, b))
    }
}

pub trait MovableListManage<T> {
    fn sort(&mut self);

    fn next_sort(&mut self);

    fn prev_sort(&mut self);

    fn sorted_merge(&mut self, other: Vec<T>);

    fn current_pos(&self) -> Coord;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;
    fn toggle(&mut self);

    fn end(&mut self);

    fn hold(&mut self);

    fn push(&mut self, item: T);

    fn handle(&mut self, event: ListEvent);
    fn offset(&self) -> &Coord;
}

impl<'a, T, S> MovableListManage<T> for MovableListState<'a, T, S>
where
    T: MovableListItem<'a>,
    S: SortMethod<T> + EndlessSelf,
{
    fn sort(&mut self) {
        if let Some(ref sort) = self.sort {
            self.items.sort_with(sort);
        }
    }

    fn next_sort(&mut self) {
        if let Some(ref mut sort) = self.sort {
            sort.next_self();
            self.items.sort_with(sort);
        }
    }

    fn prev_sort(&mut self) {
        if let Some(ref mut sort) = self.sort {
            sort.prev_self();
            self.items.sort_with(sort);
        }
    }

    fn sorted_merge(&mut self, other: Vec<T>) {
        self.items = other;
        self.sort()
    }

    fn current_pos(&self) -> Coord {
        let x = self.offset.x;
        let y = self.len().saturating_sub(self.offset.y);
        Coord {
            x,
            y,
            hold: self.offset.hold,
        }
    }

    fn len(&self) -> usize {
        self.items.len()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn toggle(&mut self) {
        self.offset.toggle()
    }

    fn end(&mut self) {
        self.offset.end()
    }

    fn hold(&mut self) {
        self.offset.hold()
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
        if self.offset.hold {
            self.offset.y += 1;
        }
    }

    fn handle(&mut self, event: ListEvent) {
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

    fn offset(&self) -> &Coord {
        &self.offset
    }
}
