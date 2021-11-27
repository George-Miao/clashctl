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
