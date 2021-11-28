use std::cmp::Ordering;

use crate::{
    interactive::{ProxySort, ProxySortBy, SortMethod, SortOrder, Sortable},
    ui::components::{ProxyGroup, ProxyItem, ProxyTree},
};

impl SortMethod<ProxyItem> for ProxySort {
    fn sort_fn(&self, a: &ProxyItem, b: &ProxyItem) -> std::cmp::Ordering {
        let cmp = match self.by() {
            ProxySortBy::Name => a.name.cmp(&b.name),
            ProxySortBy::Type => a.proxy_type.cmp(&b.proxy_type),
            ProxySortBy::Delay => {
                use Ordering::{Equal as Eq, Greater as Gt, Less as Lt};
                match (a.delay(), b.delay()) {
                    (None, Some(_)) => Gt,
                    (Some(_), None) => Lt,
                    (Some(aa), Some(bb)) => {
                        if aa == 0 {
                            Gt
                        } else if bb == 0 {
                            Lt
                        } else {
                            aa.cmp(&bb)
                        }
                    }
                    (None, None) => Eq,
                }
            }
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
    fn sort_with(&mut self, method: &ProxySort) {
        let pointed = &self.members[self.cursor].name.clone();
        let current = self.current.map(|x| self.members[x].name.clone());
        self.members.sort_by(|a, b| method.sort_fn(a, b));
        for (i, ProxyItem { name, .. }) in self.members.iter().enumerate() {
            if name == pointed {
                self.cursor = i;
            }
            if let Some(ref x) = current {
                if name == x {
                    self.current = Some(i)
                }
            }
        }
    }
}

impl<'a> Sortable<'a, ProxySort> for ProxyTree<'a> {
    type Item<'b> = ProxyItem;
    fn sort_with(&mut self, method: &ProxySort) {
        self.groups.iter_mut().for_each(|x| x.sort_with(method))
    }
}
