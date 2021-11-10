use std::env;
use std::sync::Once;

use log::info;

use crate::Clash;

static INIT: Once = Once::new();

fn init() -> Clash {
    INIT.call_once(|| simple_logger::init_with_level(log::Level::Debug).unwrap());
    Clash::builder(env::var("PROXY_ADDR").unwrap())
        .unwrap()
        .build()
}

#[test]
fn test_proxies() {
    let clash = init();
    clash.get_proxies().unwrap();
}

#[test]
fn test_proxy() {
    let clash = init();
    let proxies = clash.get_proxies().unwrap();
    let (proxy, _) = proxies.iter().next().unwrap();
    clash.get_proxy(proxy).unwrap();
}

#[test]
fn test_proxy_delay() {
    let clash = init();
    let proxies = clash.get_proxies().unwrap();
    let (proxy, _) = proxies.iter().next().unwrap();
    clash
        .get_proxy_delay(proxy, "https://google.com", 10000)
        .unwrap();
}

#[test]
fn test_set_proxy() {
    let clash = init();
    let proxies = clash.get_proxies().unwrap();
    if let Some((group, proxy)) = proxies
        .iter()
        .find(|(_, proxy)| proxy.proxy_type.is_selector())
    {
        let all = proxy.all.as_ref().unwrap();
        let member = all.iter().next().unwrap();
        clash.set_proxygroup_selected(group, member).unwrap();
    }
}

#[test]
fn test_configs() {
    let clash = init();
    clash.get_configs().unwrap();
}

#[test]
fn test_traffic() {
    let clash = init();
    clash.get_traffic().unwrap().next();
}

#[test]
fn test_log() {
    let clash = init();
    clash.get_log().unwrap().next();
}

#[test]
fn test_connections() {
    let clash = init();
    clash.get_connections().unwrap();
}

#[test]
fn test_version() {
    let clash = init();
    info!("{:#?}", clash.get_version().unwrap())
}
