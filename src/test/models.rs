use serde::de::DeserializeOwned;
use serde_json::from_str;

use crate::model::*;

async fn test_model<T: DeserializeOwned>(endpoint: &str) {
    let resp = reqwest::get(format!("http://localhost:9090{}", endpoint))
        .await
        .unwrap();
    let text = resp.text().await.unwrap();
    from_str::<T>(&text).unwrap();
}

async fn test_model_continuous<T: DeserializeOwned>(endpoint: &str) {
    let mut resp = reqwest::get(format!("http://localhost:9090{}", endpoint))
        .await
        .unwrap();

    if let Ok(Some(chunk)) = resp.chunk().await {
        let text = String::from_utf8(chunk.to_vec()).unwrap();
        from_str::<T>(&text).unwrap();
    } else {
        panic!("Unable to retrieve data")
    }
}

#[tokio::test]
async fn proxy() {
    test_model::<Proxies>("/proxies").await;
}

#[tokio::test]
async fn traffic() {
    test_model_continuous::<Traffic>("/traffic").await;
}

#[tokio::test]
async fn log() {
    test_model_continuous::<Log>("/logs").await;
}
