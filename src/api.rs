use std::io::{BufRead, BufReader};
use std::marker::PhantomData;
use std::time::Duration;

use attohttpc::body::{self};
use attohttpc::{Method, RequestBuilder, Response, ResponseReader};
use log::{debug, trace};

use serde::de::DeserializeOwned;
use serde_json::from_str;
use url::Url;

use crate::model::{Config, Delay, Proxies, Proxy, Version};
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
        if !url_str.ends_with('/') {
            url_str += "/";
        };
        let url = Url::parse(&url_str).map_err(|_| Error::UrlParseError)?;
        Ok(Self {
            url,
            secret: None,
            timeout: None,
        })
    }

    pub fn secret(mut self, secret: Option<String>) -> Self {
        self.secret = secret;
        self
    }

    pub fn timeout(mut self, timeout: Option<Duration>) -> Self {
        self.timeout = timeout;
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
        let mut req = RequestBuilder::new(method, url);

        if let Some(timeout) = self.timeout {
            req = req.timeout(timeout)
        }

        if let Some(ref secret) = self.secret {
            req = req.bearer_auth(secret)
        }

        Ok(req)
    }

    pub fn oneshot_req_with_body(
        &self,
        endpoint: &str,
        method: Method,
        body: Option<String>,
    ) -> Result<String> {
        trace!("Body: {:#?}", body);
        let resp = if let Some(body) = body {
            self.build_request(endpoint, method)?
                .body(body::Text(body))
                .send()?
        } else {
            self.build_request(endpoint, method)?.send()?
        };

        if !resp.status().is_success() {
            return Err(Error::FailedResponse(resp.status()));
        }

        let text = resp.text().map_err(|_| Error::BadResponseEncoding)?;
        trace!("Received response: {}", text);

        Ok(text)
    }

    pub fn oneshot_req(&self, endpoint: &str, method: Method) -> Result<String> {
        self.oneshot_req_with_body(endpoint, method, None)
    }

    pub fn get(&self, endpoint: &str) -> Result<String> {
        self.oneshot_req(endpoint, Method::GET)
    }

    pub fn longhaul_req<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        method: Method,
    ) -> Result<LongHaul<T>> {
        let resp = self.build_request(endpoint, method)?.send()?;

        if !resp.status().is_success() {
            return Err(Error::FailedResponse(resp.status()));
        }

        Ok(LongHaul::new(resp))
    }

    pub fn get_version(&self) -> Result<Version> {
        self.get("version").and_then(Convert::convert)
    }

    pub fn get_configs(&self) -> Result<Config> {
        self.get("configs").and_then(Convert::convert)
    }

    pub fn get_proxies(&self) -> Result<Proxies> {
        self.get("proxies").and_then(Convert::convert)
    }

    pub fn get_proxy(&self, proxy: &str) -> Result<Proxy> {
        self.get(&format!("proxies/{}", proxy))
            .and_then(Convert::convert)
    }

    pub fn set_proxygroup_selected(&self, group: &str, proxy: &str) -> Result<()> {
        let body = format!("{{\"name\":\"{}\"}}", proxy);
        self.oneshot_req_with_body(&format!("proxies/{}", group), Method::PUT, Some(body))?;
        Ok(())
    }

    pub fn get_proxy_delay(&self, proxy: &str, test_url: &str, timeout: u64) -> Result<Delay> {
        self.oneshot_req(
            &format!(
                "proxies/{}/delay?url={}&timeout={}",
                proxy, test_url, timeout
            ),
            Method::GET,
        )
        .and_then(Convert::convert)
    }
}

pub struct LongHaul<T: DeserializeOwned> {
    reader: BufReader<ResponseReader>,
    ty: PhantomData<T>,
}

impl<T: DeserializeOwned> LongHaul<T> {
    pub fn new(resp: Response) -> Self {
        let reader = BufReader::new(resp.split().2);
        Self {
            reader,
            ty: PhantomData,
        }
    }

    pub fn next_raw(&mut self) -> Option<Result<String>> {
        let mut buf = String::with_capacity(30);
        match self.reader.read_line(&mut buf) {
            Ok(0) => None,
            Ok(_) => Some(Ok(buf)),
            _ => Some(Err(Error::BadResponseEncoding)),
        }
    }
}

impl<T: DeserializeOwned> Iterator for LongHaul<T> {
    type Item = Result<T>;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_raw().map(|x| x.and_then(Convert::convert))
    }
}
