use std::cmp::Ordering;

pub trait Sortable<'a, S: SortMethod<Self::Item<'a>>> {
    type Item<'b>;
    fn sort_with(&mut self, method: S);
}

pub trait SortMethod<Item> {
    fn sort_fn(&self, a: &Item, b: &Item) -> Ordering;
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

pub struct Noop;

impl<Item> SortMethod<Item> for Noop {
    #[inline]
    fn sort_fn(&self, _: &Item, _: &Item) -> Ordering {
        Ordering::Equal
    }
}

impl<'a, T> Sortable<'a, Noop> for T {
    type Item<'b> = ();

    #[inline]
    fn sort_with(&mut self, _: Noop) {}
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

impl<'a, T, F> Sortable<'a, F> for Vec<T>
where
    F: Fn(&T, &T) -> Ordering,
{
    type Item<'b> = T;

    #[inline]
    fn sort_with(&mut self, method: F) {
        self.sort_by(method)
    }
}
