extern crate test;

use std::marker::PhantomData;

use rand::{distributions::Alphanumeric, prelude::*};
use test::Bencher;

use crate::{
    components::{ProxyGroup, ProxyItem},
    model::ProxyType,
    ui::components::ProxyTree,
};

const PROXY_TYPES: [ProxyType; 14] = [
    ProxyType::Direct,
    ProxyType::Reject,
    ProxyType::Selector,
    ProxyType::URLTest,
    ProxyType::Fallback,
    ProxyType::LoadBalance,
    ProxyType::Shadowsocks,
    ProxyType::Vmess,
    ProxyType::ShadowsocksR,
    ProxyType::Http,
    ProxyType::Snell,
    ProxyType::Trojan,
    ProxyType::Socks5,
    ProxyType::Relay,
];

fn rand_str(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

fn gen_proxy_group_type(rnd: &mut SmallRng) -> ProxyType {
    PROXY_TYPES
        .iter()
        .filter(|x| x.is_group())
        .choose(rnd)
        .unwrap()
        .to_owned()
}

fn gen_proxy_item_type(rnd: &mut SmallRng) -> ProxyType {
    PROXY_TYPES
        .iter()
        .filter(|x| x.is_normal())
        .choose(rnd)
        .unwrap()
        .to_owned()
}

fn gen_proxy_member(rnd: &mut SmallRng) -> ProxyItem {
    ProxyItem {
        name: rand_str(rnd.gen_range(5..15)),
        proxy_type: gen_proxy_item_type(rnd),
        history: None,
        udp: rnd.gen_bool(0.5),
    }
}

fn gen_proxy_group<'a>(rnd: &mut SmallRng, size: usize) -> ProxyGroup<'a> {
    let members = (0..size).map(|_| gen_proxy_member(rnd)).collect();
    ProxyGroup {
        name: rand_str(rnd.gen_range(5..15)),
        proxy_type: gen_proxy_group_type(rnd),
        current: if rnd.gen_bool(0.5) {
            Some(rnd.gen_range(0..size))
        } else {
            None
        },
        members,
        cursor: rnd.gen_range(0..size),
        _life: PhantomData,
    }
}

fn gen_proxy_tree<'a>(rnd: &mut SmallRng, size: usize) -> ProxyTree<'a> {
    let mut groups = Vec::with_capacity(size);
    for _ in 0..size {
        let size = 10;
        groups.push(gen_proxy_group(rnd, size))
    }
    ProxyTree {
        groups,
        expanded: rnd.gen_bool(0.5),
        cursor: rnd.gen_range(0..size),
    }
}

// Worst scenario where the entire proxy tree has changed
#[bench]
fn bench_proxy_tree_merge(bencher: &mut Bencher) {
    let mut rnd = SmallRng::from_entropy();
    for _ in 0..100 {
        let (mut a, b) = (gen_proxy_tree(&mut rnd, 10), gen_proxy_tree(&mut rnd, 10));
        bencher.iter(|| {
            let b = b.clone();
            a.merge(b)
        })
    }
}
