use std::marker::PhantomData;
use std::time::Duration;

use concat_idents::concat_idents;
use log::debug;
use reqwest::{Client, Method, Request, Response, Url};
use serde::de::DeserializeOwned;
use serde_json::from_str;

use crate::model::{Config, Proxies};
use crate::{Error, Result};

macro_rules! endpoint {
    ($func_name:ident, $ret_type:ty, $endpoint:expr, $method:expr) => {
        pub async fn $func_name(&self) -> Result<$ret_type> {
            self.oneshot_req($endpoint, $method).await
        }

        concat_idents!(
            fn_name = $func_name, _raw {
                pub async fn fn_name(&self) -> Result<String> {
                    self.oneshot_req_raw($endpoint, $method).await
                }

            }
        );
    };
}

#[derive(Debug, Clone)]
pub struct ClashBuilder {
    url: Url,
    secret: Option<String>,
    timeout: Option<Duration>,
}

impl ClashBuilder {
    pub fn new<S: Into<String>>(url: S) -> Result<Self> {
        let mut url_str = url.into();
        // Handle trailling slash
        if !url_str.ends_with("/") {
            url_str += "/";
        };
        let url = Url::parse(&url_str).map_err(|_| Error::UrlParseError)?;
        Ok(Self {
            url,
            secret: None,
            timeout: None,
        })
    }

    pub fn secret(mut self, secret: String) -> Self {
        self.secret = Some(secret);
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn build(self) -> Clash {
        let mut clash = Clash::new(self.url);
        clash.secret = self.secret;
        clash.timeout = self.timeout;
        clash
    }
}

#[derive(Debug)]
pub struct Clash {
    url: Url,
    secret: Option<String>,
    timeout: Option<Duration>,
}

impl Clash {
    pub fn builder<S: Into<String>>(url: S) -> Result<ClashBuilder> {
        ClashBuilder::new(url)
    }

    pub fn new(url: Url) -> Self {
        debug!("Url of clash RESTful API: {}", url);
        Self {
            url,
            secret: None,
            timeout: None,
        }
    }

    fn build_request(&self, endpoint: &str, method: Method) -> Result<Request> {
        let url = self.url.join(endpoint).map_err(|_| Error::UrlParseError)?;
        let mut req = Client::new().request(method, url);

        if let Some(timeout) = self.timeout {
            req = req.timeout(timeout)
        }

        if let Some(ref secret) = self.secret {
            req = req.bearer_auth(secret)
        }

        Ok(req.build().unwrap())
    }

    pub async fn oneshot_req_raw(&self, endpoint: &str, method: Method) -> Result<String> {
        Ok(Client::new()
            .execute(self.build_request(endpoint, method)?)
            .await
            .map_err(|_| Error::RequestError)?
            .text()
            .await
            .map_err(|_| Error::BadResponseEncoding)?)
    }

    pub async fn oneshot_req<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        method: Method,
    ) -> Result<T> {
        from_str(&self.oneshot_req_raw(endpoint, method).await?)
            .map_err(|_| Error::BadResponseFormat)
    }

    pub async fn longhaul_req_raw(&self, endpoint: &str, method: Method) -> Result<RawLongHaul> {
        let resp = Client::new()
            .execute(self.build_request(endpoint, method)?)
            .await
            .map_err(|_| Error::RequestError)?;
        Ok(RawLongHaul { resp })
    }

    pub async fn longhaul_req<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        method: Method,
    ) -> Result<LongHaul<T>> {
        let resp = Client::new()
            .execute(self.build_request(endpoint, method)?)
            .await
            .map_err(|_| Error::RequestError)?;
        Ok(LongHaul {
            resp,
            ty: PhantomData,
        })
    }

    endpoint!(get_proxies, Proxies, "proxies", Method::GET);

    endpoint!(get_configs, Config, "configs", Method::GET);
}

pub struct RawLongHaul {
    resp: Response,
}

impl RawLongHaul {
    pub async fn next(&mut self) -> Result<Option<String>> {
        match self.resp.chunk().await {
            Ok(Some(byte)) => Ok(Some(
                std::str::from_utf8(&byte)
                    .map_err(|_| Error::BadResponseEncoding)?
                    .to_string(),
            )),
            Ok(None) => Ok(None),
            Err(_) => Err(Error::BadResponseEncoding),
        }
    }
}

pub struct LongHaul<T: DeserializeOwned> {
    resp: Response,
    ty: PhantomData<T>,
}

impl<T: DeserializeOwned> LongHaul<T> {
    pub async fn next(&mut self) -> Result<Option<T>> {
        match self.resp.chunk().await {
            Ok(Some(byte)) => {
                let text = std::str::from_utf8(&byte).map_err(|_| Error::BadResponseEncoding)?;
                Ok(from_str(text).map_err(|_| Error::BadResponseFormat)?)
            }
            Ok(None) => Ok(None),
            Err(_) => Err(Error::BadResponseEncoding),
        }
    }
}

#[tokio::test]
async fn test_proxies() {
    let clash = Clash::builder("http://proxy.lan:9090").unwrap().build();
    let resp = clash.get_proxies_raw().await.unwrap();
    println!("{:#?}", resp);
    let resp = clash.get_proxies().await.unwrap();
    println!("{:#?}", resp);
}

#[tokio::test]
async fn test_configs() {
    let clash = Clash::builder("http://proxy.lan:9090").unwrap().build();
    let resp = clash.get_configs_raw().await.unwrap();
    println!("{:#?}", resp);
    let resp = clash.get_configs().await.unwrap();
    println!("{:#?}", resp);
}
