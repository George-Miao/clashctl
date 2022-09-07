use std::{
    borrow::Cow,
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crossterm::event::KeyCode;
use match_any::match_any;
use paste::paste;
use smart_default::SmartDefault;

use clashctl_interactive::{EndlessSelf, SortMethod, Sortable};

use crate::{
    components::{MovableListItem, ProxyTree},
    utils::Coord,
    Action, ConListState, DebugListState, ListEvent, LogListState, RuleListState,
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

impl<'a, T, S> MovableListState<'a, T, S>
where
    T: MovableListItem<'a>,
    S: SortMethod<T> + EndlessSelf + Default,
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
            sort,
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

    pub fn sorted_merge(&mut self, other: Vec<T>) {
        self.items = other;
        self.sort();
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
        if self.offset.hold {
            self.offset.y += 1;
        }
    }
}

// TODO: Use lazy updated footer
#[derive(Debug, Clone, PartialEq, Eq, SmartDefault)]
pub struct MovableListState<'a, T: MovableListItem<'a>, S: Default> {
    pub(super) offset: Coord,
    pub(super) items: Vec<T>,
    pub(super) placeholder: Option<Cow<'a, str>>,
    #[default = 1]
    pub(super) padding: u16,
    pub(super) sort: S,
    pub(super) with_index: bool,
    pub(super) reverse_index: bool,
}

impl<'a, T, S> Deref for MovableListState<'a, T, S>
where
    T: MovableListItem<'a>,
    S: Default,
{
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl<'a, T, S> DerefMut for MovableListState<'a, T, S>
where
    T: MovableListItem<'a>,
    S: Default,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

impl<'a, T, S> Extend<T> for MovableListState<'a, T, S>
where
    T: MovableListItem<'a>,
    S: Default,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.items.extend(iter)
    }
}

impl<'a, T, S> Sortable<'a, S> for MovableListState<'a, T, S>
where
    T: MovableListItem<'a>,
    S: SortMethod<T> + Default,
{
    type Item<'b> = T;
    fn sort_with(&mut self, method: &S) {
        self.items.sort_by(|a, b| method.sort_fn(a, b))
    }
}

pub trait MovableListManage {
    fn sort(&mut self) -> &mut Self;

    fn next_sort(&mut self) -> &mut Self;

    fn prev_sort(&mut self) -> &mut Self;

    fn current_pos(&self) -> Coord;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;
    fn toggle(&mut self) -> &mut Self;

    fn end(&mut self) -> &mut Self;

    fn hold(&mut self) -> &mut Self;

    fn handle(&mut self, event: ListEvent) -> Option<Action>;
    fn offset(&self) -> &Coord;
}

impl<'a, T, S> MovableListManage for MovableListState<'a, T, S>
where
    T: MovableListItem<'a>,
    S: SortMethod<T> + EndlessSelf + Default,
{
    fn sort(&mut self) -> &mut Self {
        let sort = &self.sort;
        self.items.sort_with(sort);
        self
    }

    fn next_sort(&mut self) -> &mut Self {
        self.sort.next_self();
        let sort = &self.sort;
        self.items.sort_with(sort);
        self
    }

    fn prev_sort(&mut self) -> &mut Self {
        self.sort.prev_self();
        let sort = &self.sort;
        self.items.sort_with(sort);
        self
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

    fn toggle(&mut self) -> &mut Self {
        self.offset.toggle();
        self
    }

    fn end(&mut self) -> &mut Self {
        self.offset.end();
        self
    }

    fn hold(&mut self) -> &mut Self {
        self.offset.hold();
        self
    }

    fn handle(&mut self, event: ListEvent) -> Option<Action> {
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
        None
    }

    fn offset(&self) -> &Coord {
        &self.offset
    }
}

pub enum MovableListManager<'a, 'own> {
    Log(&'own mut LogListState<'a>),
    Connection(&'own mut ConListState<'a>),
    Rule(&'own mut RuleListState<'a>),
    Event(&'own mut DebugListState<'a>),
    Proxy(&'own mut ProxyTree<'a>),
}

impl<'a, 'own> MovableListManage for MovableListManager<'a, 'own> {
    fn sort(&mut self) -> &mut Self {
        match_any!(
            self,
            Self::Log(inner) |
            Self::Event(inner) |
            Self::Rule(inner) |
            Self::Connection(inner) |
            Self::Proxy(inner) => {
                inner.sort();
            }
        );
        self
    }

    fn next_sort(&mut self) -> &mut Self {
        match_any!(
            self,
            Self::Log(inner) |
            Self::Event(inner) |
            Self::Rule(inner) |
            Self::Connection(inner) |
            Self::Proxy(inner) => {
                inner.next_sort();
            }
        );
        self
    }

    fn prev_sort(&mut self) -> &mut Self {
        match_any!(
            self,
            Self::Log(inner) |
            Self::Event(inner) |
            Self::Rule(inner) |
            Self::Connection(inner) |
            Self::Proxy(inner) => {
                inner.prev_sort();
            }
        );
        self
    }

    fn current_pos(&self) -> Coord {
        match_any!(
            self,
            Self::Log(inner) |
            Self::Event(inner) |
            Self::Rule(inner) |
            Self::Connection(inner) |
            Self::Proxy(inner) => {
                inner.current_pos()
            }
        )
    }

    fn len(&self) -> usize {
        match_any!(
            self,
            Self::Log(inner) |
            Self::Event(inner) |
            Self::Rule(inner) |
            Self::Connection(inner) |
            Self::Proxy(inner) => {
                inner.len()
            }
        )
    }

    fn is_empty(&self) -> bool {
        match_any!(
            self,
            Self::Log(inner) |
            Self::Event(inner) |
            Self::Rule(inner) |
            Self::Connection(inner) |
            Self::Proxy(inner) => {
                inner.is_empty()
            }
        )
    }

    fn toggle(&mut self) -> &mut Self {
        match_any!(
            self,
            Self::Log(inner) |
            Self::Event(inner) |
            Self::Rule(inner) |
            Self::Connection(inner) |
            Self::Proxy(inner) => {
                inner.toggle();
            }
        );
        self
    }

    fn end(&mut self) -> &mut Self {
        match_any!(
            self,
            Self::Log(inner) |
            Self::Event(inner) |
            Self::Rule(inner) |
            Self::Connection(inner) |
            Self::Proxy(inner) => {
                inner.end();
            }
        );
        self
    }

    fn hold(&mut self) -> &mut Self {
        match_any!(
            self,
            Self::Log(inner) |
            Self::Event(inner) |
            Self::Rule(inner) |
            Self::Connection(inner) |
            Self::Proxy(inner) => {
                inner.hold();
            }
        );
        self
    }

    fn handle(&mut self, event: ListEvent) -> Option<Action> {
        match_any!(
            self,
            Self::Log(inner) |
            Self::Event(inner) |
            Self::Rule(inner) |
            Self::Connection(inner) |
            Self::Proxy(inner) => {
                inner.handle(event)
            }
        )
    }

    fn offset(&self) -> &Coord {
        match_any!(
            self,
            Self::Log(inner) |
            Self::Event(inner) |
            Self::Rule(inner) |
            Self::Connection(inner) |
            Self::Proxy(inner) => {
                inner.offset()
            }
        )
    }
}
