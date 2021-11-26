use std::io::{BufRead, BufReader, Read};
use std::marker::PhantomData;
use std::time::Duration;

use log::{debug, trace};

use serde::de::DeserializeOwned;
use serde_json::from_str;
use ureq::{Agent, Request};
use url::Url;

use crate::model::{Config, Connections, Delay, Log, Proxies, Proxy, Rules, Traffic, Version};
use crate::{Error, Result};

trait Convert<T: DeserializeOwned> {
    fn convert(self) -> Result<T>;
}

impl<T: DeserializeOwned> Convert<T> for String {
    fn convert(self) -> Result<T> {
        from_str(&self).map_err(Error::BadResponseFormat)
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

#[derive(Debug, Clone)]
pub struct Clash {
    url: Url,
    secret: Option<String>,
    timeout: Option<Duration>,
    agent: Agent,
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
            agent: Agent::new(),
        }
    }

    fn build_request(&self, endpoint: &str, method: &str) -> Result<Request> {
        let url = self.url.join(endpoint).map_err(|_| Error::UrlParseError)?;
        let mut req = self.agent.request_url(method, &url);

        if let Some(timeout) = self.timeout {
            req = req.timeout(timeout)
        }

        if let Some(ref secret) = self.secret {
            req = req.set("Authorization", &format!("Bearer {}", secret))
        }

        Ok(req)
    }

    fn build_request_without_timeout(&self, endpoint: &str, method: &str) -> Result<Request> {
        let url = self.url.join(endpoint).map_err(|_| Error::UrlParseError)?;
        let mut req = self.agent.request_url(method, &url);

        if let Some(ref secret) = self.secret {
            req = req.set("Authorization", &format!("Bearer {}", secret))
        }

        Ok(req)
    }

    pub fn oneshot_req_with_body(
        &self,
        endpoint: &str,
        method: &str,
        body: Option<String>,
    ) -> Result<String> {
        trace!("Body: {:#?}", body);
        let resp = if let Some(body) = body {
            self.build_request(endpoint, method)?.send_string(&body)?
        } else {
            self.build_request(endpoint, method)?.call()?
        };

        if resp.status() >= 400 {
            return Err(Error::FailedResponse(resp.status()));
        }

        let text = resp.into_string().map_err(|_| Error::BadResponseEncoding)?;
        trace!("Received response: {}", text);

        Ok(text)
    }

    pub fn oneshot_req(&self, endpoint: &str, method: &str) -> Result<String> {
        self.oneshot_req_with_body(endpoint, method, None)
    }

    pub fn longhaul_req<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        method: &str,
    ) -> Result<LongHaul<T>> {
        let resp = self
            .build_request_without_timeout(endpoint, method)?
            .call()?;

        if resp.status() >= 400 {
            return Err(Error::FailedResponse(resp.status()));
        }

        Ok(LongHaul::new(Box::new(resp.into_reader())))
    }

    pub fn get<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T> {
        self.oneshot_req(endpoint, "GET").and_then(Convert::convert)
    }

    pub fn get_version(&self) -> Result<Version> {
        self.get("version")
    }

    pub fn get_configs(&self) -> Result<Config> {
        self.get("configs")
    }

    pub fn get_proxies(&self) -> Result<Proxies> {
        self.get("proxies")
    }

    pub fn get_rules(&self) -> Result<Rules> {
        self.get("rules")
    }

    pub fn get_proxy(&self, proxy: &str) -> Result<Proxy> {
        self.get(&format!("proxies/{}", proxy))
    }

    pub fn get_connections(&self) -> Result<Connections> {
        self.get("connections")
    }

    pub fn get_traffic(&self) -> Result<LongHaul<Traffic>> {
        self.longhaul_req("traffic", "GET")
    }

    pub fn get_log(&self) -> Result<LongHaul<Log>> {
        self.longhaul_req("logs", "GET")
    }

    pub fn get_proxy_delay(&self, proxy: &str, test_url: &str, timeout: u64) -> Result<Delay> {
        self.get(&format!(
            "proxies/{}/delay?url={}&timeout={}",
            proxy, test_url, timeout
        ))
    }

    pub fn set_proxygroup_selected(&self, group: &str, proxy: &str) -> Result<()> {
        let body = format!("{{\"name\":\"{}\"}}", proxy);
        self.oneshot_req_with_body(&format!("proxies/{}", group), "PUT", Some(body))?;
        Ok(())
    }
}

pub struct LongHaul<T: DeserializeOwned> {
    reader: BufReader<Box<dyn Read + Send>>,
    ty: PhantomData<T>,
}

impl<T: DeserializeOwned> LongHaul<T> {
    pub fn new(reader: Box<dyn Read + Send>) -> Self {
        Self {
            reader: BufReader::new(reader),
            ty: PhantomData,
        }
    }

    pub fn next_item(&mut self) -> Option<Result<T>> {
        self.next_raw().map(|x| x.and_then(Convert::convert))
    }

    pub fn next_raw(&mut self) -> Option<Result<String>> {
        let mut buf = String::with_capacity(30);
        match self.reader.read_line(&mut buf) {
            Ok(0) => None,
            Ok(_) => Some(Ok(buf)),
            Err(e) => Some(Err(Error::Other(format!("{:}", e)))),
            // _ => Some(Err(Error::BadResponseEncoding)),
        }
    }
}

impl<T: DeserializeOwned> Iterator for LongHaul<T> {
    type Item = Result<T>;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_item()
    }
}
