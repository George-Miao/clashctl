use std::marker::PhantomData;
use std::time::Duration;

use log::debug;
use reqwest::{Client, Method, RequestBuilder, Response, Url};
use serde::de::DeserializeOwned;
use serde_json::from_str;

use crate::model::{Config, Delay, Proxies, Proxy};
use crate::{Error, Result};

trait Convert<T: DeserializeOwned> {
    fn convert(self) -> Result<T>;
}

impl<T: DeserializeOwned> Convert<T> for String {
    fn convert(self) -> Result<T> {
        from_str(&self).map_err(|_| Error::BadResponseFormat)
    }
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

    fn build_request(&self, endpoint: &str, method: Method) -> Result<RequestBuilder> {
        let url = self.url.join(endpoint).map_err(|_| Error::UrlParseError)?;
        let mut req = Client::new().request(method, url);

        if let Some(timeout) = self.timeout {
            req = req.timeout(timeout)
        }

        if let Some(ref secret) = self.secret {
            req = req.bearer_auth(secret)
        }

        Ok(req)
    }

    pub async fn oneshot_req_with_body(
        &self,
        endpoint: &str,
        method: Method,
        body: Option<String>,
    ) -> Result<String> {
        debug!("Body: {:#?}", body);
        let resp = if let Some(body) = body {
            self.build_request(endpoint, method)?.body(body)
        } else {
            self.build_request(endpoint, method)?
        }
        .send()
        .await
        .map_err(|_| Error::BadResponseEncoding)?;

        if !resp.status().is_success() {
            return Err(Error::FailedResponse(resp.status()));
        }

        let text = resp.text().await.map_err(|_| Error::BadResponseEncoding)?;
        debug!("Received response: {}", text);

        Ok(text)
    }

    pub async fn oneshot_req(&self, endpoint: &str, method: Method) -> Result<String> {
        self.oneshot_req_with_body(endpoint, method, None).await
    }

    pub async fn longhaul_req<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        method: Method,
    ) -> Result<LongHaul<T>> {
        let resp = Client::new()
            .execute(self.build_request(endpoint, method)?.build().unwrap())
            .await?;

        if !resp.status().is_success() {
            return Err(Error::FailedResponse(resp.status()));
        }

        Ok(LongHaul {
            resp,
            ty: PhantomData,
        })
    }

    pub async fn get_configs(&self) -> Result<Config> {
        self.oneshot_req("configs", Method::GET)
            .await
            .and_then(Convert::convert)
    }

    pub async fn get_proxies(&self) -> Result<Proxies> {
        self.oneshot_req("proxies", Method::GET)
            .await
            .and_then(Convert::convert)
    }

    pub async fn get_proxy(&self, proxy: &str) -> Result<Proxy> {
        self.oneshot_req(&format!("proxies/{}", proxy), Method::GET)
            .await
            .and_then(Convert::convert)
    }

    pub async fn set_proxygroup_selected(&self, group: &str, proxy: &str) -> Result<()> {
        let body = format!("{{\"name\":\"{}\"}}", proxy);
        self.oneshot_req_with_body(&format!("proxies/{}", group), Method::PUT, Some(body))
            .await?;
        Ok(())
    }

    pub async fn get_proxy_delay(
        &self,
        proxy: &str,
        test_url: &str,
        timeout: u64,
    ) -> Result<Delay> {
        self.oneshot_req(
            &format!(
                "proxies/{}/delay?url={}&timeout={}",
                proxy, test_url, timeout
            ),
            Method::GET,
        )
        .await
        .and_then(Convert::convert)
    }
}

pub struct LongHaul<T: DeserializeOwned> {
    resp: Response,
    ty: PhantomData<T>,
}

impl<T: DeserializeOwned> LongHaul<T> {
    pub async fn next(&mut self) -> Result<Option<T>> {
        match self.next_raw().await {
            Ok(Some(text)) => text.convert(),
            Ok(None) => Ok(None),
            Err(_) => Err(Error::BadResponseEncoding),
        }
    }

    pub async fn next_raw(&mut self) -> Result<Option<String>> {
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
