use crate::{
    interactive::{ProxySort, ProxySortBy, SortMethod, SortOrder, Sortable},
    ui::components::{ProxyGroup, ProxyItem, ProxyTree},
};

// impl<'a> ProxyGroup<'a> {
//     pub fn sort_by(&mut self, sort_method: ProxySort) {
//         let vec = &mut self.members;
//         match sort_method.by {
//             ProxySortBy::Name => vec.sort_by(|a, b| a.name.cmp(&b.name)),
//             ProxySortBy::Type => vec.sort_by_cached_key(|x| x.proxy_type),
//             ProxySortBy::Delay => vec.sort_by_cached_key(|x| x.delay()),
//         };
//         if matches!(sort_method.order, SortOrder::Descendant) {
//             vec.reverse()
//         }
//     }
// }

// impl<'a> ProxyTree<'a> {
//     pub fn sort_groups_by(&mut self, sort_method: ProxySort) {
//         self.groups.iter_mut().for_each(|x| x.sort_by(sort_method))
//     }
// }

impl SortMethod<ProxyItem> for ProxySort {
    fn sort_fn(&self, a: &ProxyItem, b: &ProxyItem) -> std::cmp::Ordering {
        let cmp = match self.by() {
            ProxySortBy::Name => a.name.cmp(&b.name),
            ProxySortBy::Type => a.proxy_type.cmp(&b.proxy_type),
            ProxySortBy::Delay => a.delay().cmp(&b.delay()),
        };
        if matches!(self.order(), SortOrder::Descendant) {
            cmp.reverse()
        } else {
            cmp
        }
    }
}

impl<'a> Sortable<'a, ProxySort> for ProxyGroup<'a> {
    type Item<'b> = ProxyItem;
    fn sort_with(&mut self, method: ProxySort) {
        self.members.sort_by(|a, b| method.sort_fn(a, b))
    }
}

impl<'a> Sortable<'a, ProxySort> for ProxyTree<'a> {
    type Item<'b> = ProxyItem;
    fn sort_with(&mut self, method: ProxySort) {
        self.groups.iter_mut().for_each(|x| x.sort_with(method))
    }
}
