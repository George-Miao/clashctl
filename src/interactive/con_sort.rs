use strum::IntoEnumIterator;

use crate::interactive::{EndlessSelf, OrderBy, SortMethod, SortOrder};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::EnumIter)]
enum ConSortBy {
    Host,
    Down,
    Up,
    DownSpeed,
    UpSpeed,
    Chains,
    Rule,
    Time,
    Src,
    Dest,
    Type,
}

impl SortMethod<ConnectionWithSpeed> for ConSortBy {
    fn sort_fn(&self, a: &ConnectionWithSpeed, b: &ConnectionWithSpeed) -> std::cmp::Ordering {
        todo!()
    }
}

impl EndlessSelf for ConSortBy {
    fn next_self(&mut self) {
        todo!()
    }

    fn prev_self(&mut self) {
        todo!()
    }
}

pub struct ConSort<I>
where
    I: Iterator<Item = ConSortBy>,
{
    iter: I,
    by: ConSortBy,
    order: SortOrder,
}

impl<I> EndlessSelf for ConSort<I>
where
    I: Iterator<Item = ConSortBy>,
{
    fn next_self(&mut self) {
        let next = self.iter.next().unwrap();
        self.by = next;
        todo!()
    }

    fn prev_self(&mut self) {
        todo!()
    }
}

impl SortMethod<ConnectionWithSpeed> for ConSort {
    fn sort_fn(&self, a: &ConnectionWithSpeed, b: &ConnectionWithSpeed) -> std::cmp::Ordering {
        self.by.sort_fn(a, b).order_by(self.order)
    }
}

use crate::model::ConnectionWithSpeed;
