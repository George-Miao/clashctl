use std::{env, sync::Once};

use home::home_dir;
use log::info;

use crate::Clash;

static INIT: Once = Once::new();

fn init() -> Clash {
    INIT.call_once(|| {
        if env::var("RUST_LOG").is_err() {
            env::set_var("RUST_LOG", "DEBUG")
        }
        pretty_env_logger::init()
    });
    Clash::builder(env::var("PROXY_ADDR").unwrap())
        .unwrap()
        .secret(env::var("PROXY_SECRET").ok())
        .build()
}

#[test]
fn test_proxies() {
    let clash = init();
    clash.get_proxies().unwrap();
}

#[test]
fn test_rules() {
    let clash = init();
    clash.get_rules().unwrap();
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
    let (proxy, _) = proxies.iter().find(|x| x.1.proxy_type.is_normal()).unwrap();
    clash
        .get_proxy_delay(proxy, "https://static.miao.dev/generate_204", 10000)
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
    let default_config_dir = home_dir()
        .expect("Home dir should exist")
        .join(".config/clash/config.yaml");
    let path = default_config_dir.to_str().unwrap();

    clash.get_configs().unwrap();
    // clash.reload_configs(false, path).unwrap();
    // clash.reload_configs(true, path).unwrap();
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
    let cons = clash.get_connections().unwrap();
    let res = &cons
        .connections
        .first()
        .expect("Should exist at least one connection")
        .id;
    clash.close_one_connection(res).unwrap();
    clash.close_connections().unwrap();
}

#[test]
fn test_version() {
    let clash = init();
    info!("{:#?}", clash.get_version().unwrap())
}
