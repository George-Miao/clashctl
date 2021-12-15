use clashctl_core::model::ConnectionWithSpeed;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

use crate::{EndlessSelf, OrderBy, SortMethod, SortOrder};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    SmartDefault,
    strum::EnumIter,
)]
#[serde(rename_all = "lowercase")]
enum ConSortBy {
    Host,
    Down,
    Up,
    DownSpeed,
    UpSpeed,
    Chains,
    Rule,
    #[default]
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub struct ConSort {
    by: ConSortBy,
    order: SortOrder,
}

impl EndlessSelf for ConSort {
    fn next_self(&mut self) {
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
