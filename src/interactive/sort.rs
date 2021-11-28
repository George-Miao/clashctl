use std::cmp::Ordering;

pub trait Sortable<'a, S: SortMethod<Self::Item<'a>>> {
    type Item<'b>;
    fn sort_with(&mut self, method: &S);
}

pub trait SortMethod<Item> {
    fn sort_fn(&self, a: &Item, b: &Item) -> Ordering;
}

pub trait EndlessSelf {
    fn next_self(self) -> Self;
    fn prev_self(self) -> Self;
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    strum::EnumString,
    strum::Display,
    strum::EnumVariantNames,
)]
#[strum(ascii_case_insensitive)]
pub enum SortOrder {
    Ascendant,
    Descendant,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Default, Hash)]
pub struct Noop;

impl Noop {
    pub const fn new() -> Self {
        Noop
    }
}

impl<Item> SortMethod<Item> for Noop {
    #[inline]
    fn sort_fn(&self, _: &Item, _: &Item) -> Ordering {
        Ordering::Equal
    }
}

impl EndlessSelf for Noop {
    fn next_self(self) -> Self {
        Noop
    }

    fn prev_self(self) -> Self {
        Noop
    }
}

impl<T, F> SortMethod<T> for F
where
    F: Fn(&T, &T) -> Ordering,
{
    #[inline]
    fn sort_fn(&self, a: &T, b: &T) -> Ordering {
        self(a, b)
    }
}

impl<'a, T, M> Sortable<'a, M> for Vec<T>
where
    M: SortMethod<T>,
{
    type Item<'b> = T;

    #[inline]
    fn sort_with(&mut self, method: &M) {
        self.sort_by(|a, b| method.sort_fn(a, b))
    }
}
