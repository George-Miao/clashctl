use std::env;

use log::info;

use crate::Clash;

fn init() -> Clash {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    Clash::builder(env::var("PROXY_ADDR").unwrap())
        .unwrap()
        .build()
}

#[tokio::test]
async fn test_proxies() {
    let clash = init();
    clash.get_proxies().await.unwrap();
}

#[tokio::test]
async fn test_proxy() {
    let clash = init();
    let proxies = clash.get_proxies().await.unwrap();
    let (proxy, _) = proxies.iter().next().unwrap();
    clash.get_proxy(proxy).await.unwrap();
}

#[tokio::test]
async fn test_proxy_delay() {
    let clash = init();
    let proxies = clash.get_proxies().await.unwrap();
    let (proxy, _) = proxies.iter().next().unwrap();
    clash
        .get_proxy_delay(proxy, "https://google.com", 10000)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_set_proxy() {
    let clash = init();
    let proxies = clash.get_proxies().await.unwrap();
    if let Some((group, proxy)) = proxies
        .iter()
        .find(|(_, proxy)| proxy.proxy_type.is_selector())
    {
        let all = proxy.all.as_ref().unwrap();
        let member = all.iter().next().unwrap();
        clash.set_proxygroup_selected(group, member).await.unwrap();
    }
}

#[tokio::test]
async fn test_configs() {
    let clash = init();
    clash.get_configs().await.unwrap();
}

#[tokio::test]
async fn test_version() {
    let clash = init();
    info!("{:#?}", clash.get_version().await.unwrap())
}
