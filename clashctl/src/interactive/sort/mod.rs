use std::cmp::Ordering;

use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

mod_use::mod_use![con_sort, proxy_sort, rule_sort];

pub trait Sortable<'a, S: SortMethod<Self::Item<'a>>> {
    type Item<'b>;
    fn sort_with(&mut self, method: &S);
}

pub trait SortMethod<Item> {
    fn sort_fn(&self, a: &Item, b: &Item) -> Ordering;
}

pub trait EndlessSelf {
    fn next_self(&mut self);
    fn prev_self(&mut self);
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    SmartDefault,
    strum::EnumString,
    strum::Display,
    strum::EnumVariantNames,
)]
#[serde(rename_all = "lowercase")]
#[strum(ascii_case_insensitive)]
pub enum SortOrder {
    Ascendant,
    #[default]
    Descendant,
}

pub trait OrderBy {
    fn order_by(self, order: SortOrder) -> Ordering;
}

impl OrderBy for Ordering {
    fn order_by(self, order: SortOrder) -> Ordering {
        if matches!(order, SortOrder::Descendant) {
            self.reverse()
        } else {
            self
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Default, Hash)]
pub struct Noop;

impl Noop {
    pub const fn new() -> Self {
        Noop
    }
}

impl ToString for Noop {
    fn to_string(&self) -> String {
        "".into()
    }
}

impl<Item> SortMethod<Item> for Noop {
    #[inline]
    fn sort_fn(&self, _: &Item, _: &Item) -> Ordering {
        Ordering::Equal
    }
}

impl EndlessSelf for Noop {
    fn next_self(&mut self) {}

    fn prev_self(&mut self) {}
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

// #[macro_export]
// macro_rules! endless {
//     ( $ty:path = $from:ident => $( $to:ident $(=>)? )+ ) => {
//         impl EndlessSelf for $ty {
//             fn next_self(&mut self) {
//                 use $ty::*;
//                 match self {
//                     endless!( @inner $ty = $from => $( $to => )+ )
//                 }
//             }
//             fn prev_self(&mut self) {}
//         }
//     };
//     ( @inner $ty:path = $prev:ident => $from:ident => $( $to:ident $(=>)? )+
// ) => {         $ty::$prev => $ty::$from,
//         endless!(@inner $ty = $from => $($to =>)+),
//     };
//     ( @inner $ty:path = $from:ident => $to:ident ) => {
//         $from => $to,
//     }
// }

// endless!( RuleSortBy = Payload => Proxy => Type );
