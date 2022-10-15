use std::{
    io::{BufRead, BufReader, Read},
    marker::PhantomData,
    time::Duration,
};

use log::{debug, trace};
use serde::de::DeserializeOwned;
use serde_json::{from_str, json};
use ureq::{Agent, Request};
use url::Url;

use crate::{
    model::{Config, Connections, Delay, Log, Proxies, Proxy, Rules, Traffic, Version},
    Error, Result,
};

trait Convert<T: DeserializeOwned> {
    fn convert(self) -> Result<T>;
}

impl<T: DeserializeOwned> Convert<T> for String {
    fn convert(self) -> Result<T> {
        from_str(&self).map_err(Into::into)
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
        let url = Url::parse(&url_str).map_err(|_| Error::url_parse())?;
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

/// # Clash API
///
/// Use struct `Clash` for interacting with Clash RESTful API.
/// For more information, check <https://github.com/Dreamacro/clash/wiki/external-controller-API-reference###Proxies>,
/// or maybe just read source code of clash
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
        let url = self.url.join(endpoint).map_err(|_| Error::url_parse())?;
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
        let url = self.url.join(endpoint).map_err(|_| Error::url_parse())?;
        let mut req = self.agent.request_url(method, &url);

        if let Some(ref secret) = self.secret {
            req = req.set("Authorization", &format!("Bearer {}", secret))
        }

        Ok(req)
    }

    /// Send a oneshot request to the specific endpoint with method, with body
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
            return Err(Error::failed_response(resp.status()));
        }

        let text = resp
            .into_string()
            .map_err(|_| Error::bad_response_encoding())?;
        trace!("Received response: {}", text);

        Ok(text)
    }

    /// Send a oneshot request to the specific endpoint with method, without
    /// body
    pub fn oneshot_req(&self, endpoint: &str, method: &str) -> Result<String> {
        self.oneshot_req_with_body(endpoint, method, None)
    }

    /// Send a longhaul request to the specific endpoint with method,
    /// Underlying is an http stream with chunked-encoding.
    ///
    /// Use [`LongHaul::next_item`], [`LongHaul::next_raw`] or
    /// [`Iterator::next`] to retreive data
    ///
    /// # Examplel
    ///
    /// ```rust
    /// # use clashctl_core::{ Clash, model::Traffic }; use std::env;
    /// # fn main() {
    /// # let clash = Clash::builder(env::var("PROXY_ADDR").unwrap()).unwrap().build();
    /// let traffics = clash
    ///     .longhaul_req::<Traffic>("traffic", "GET")
    ///     .expect("connect failed");
    ///
    /// // LongHaul implements `Iterator` so you can use iterator combinators
    /// for traffic in traffics.take(3) {
    ///     println!("{:#?}", traffic)
    /// }
    /// # }
    /// ```
    pub fn longhaul_req<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        method: &str,
    ) -> Result<LongHaul<T>> {
        let resp = self
            .build_request_without_timeout(endpoint, method)?
            .call()?;

        if resp.status() >= 400 {
            return Err(Error::failed_response(resp.status()));
        }

        Ok(LongHaul::new(Box::new(resp.into_reader())))
    }

    /// Helper function for method `GET`
    pub fn get<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T> {
        self.oneshot_req(endpoint, "GET").and_then(Convert::convert)
    }

    /// Helper function for method `DELETE`
    pub fn delete(&self, endpoint: &str) -> Result<()> {
        self.oneshot_req(endpoint, "DELETE").map(|_| ())
    }

    /// Helper function for method `PUT`
    pub fn put<T: DeserializeOwned>(&self, endpoint: &str, body: Option<String>) -> Result<T> {
        self.oneshot_req_with_body(endpoint, "PUT", body)
            .and_then(Convert::convert)
    }

    /// Get clash version
    pub fn get_version(&self) -> Result<Version> {
        self.get("version")
    }

    /// Get base configs
    pub fn get_configs(&self) -> Result<Config> {
        self.get("configs")
    }

    /// Reloading base configs.
    ///
    /// - `force`: will change ports etc.,
    /// - `path`: the absolute path to config file
    ///
    /// This will **NOT** affect `external-controller` & `secret`
    pub fn reload_configs(&self, force: bool, path: &str) -> Result<()> {
        let body = json!({ "path": path }).to_string();
        debug!("{}", body);
        self.put::<String>(if force { "configs?force" } else { "configs" }, Some(body))
            .map(|_| ())
    }

    /// Get proxies information
    pub fn get_proxies(&self) -> Result<Proxies> {
        self.get("proxies")
    }

    /// Get rules information
    pub fn get_rules(&self) -> Result<Rules> {
        self.get("rules")
    }

    /// Get specific proxy information
    pub fn get_proxy(&self, proxy: &str) -> Result<Proxy> {
        self.get(&format!("proxies/{}", proxy))
    }

    /// Get connections information
    pub fn get_connections(&self) -> Result<Connections> {
        self.get("connections")
    }

    /// Close all connections
    pub fn close_connections(&self) -> Result<()> {
        self.delete("connections")
    }

    /// Close specific connection
    pub fn close_one_connection(&self, id: &str) -> Result<()> {
        self.delete(&format!("connections/{}", id))
    }

    /// Get real-time traffic data
    ///
    /// **Note**: This is a longhaul request, which will last forever until
    /// interrupted or disconnected.
    ///
    /// See [`longhaul_req`] for more information
    ///
    /// [`longhaul_req`]: Clash::longhaul_req
    pub fn get_traffic(&self) -> Result<LongHaul<Traffic>> {
        self.longhaul_req("traffic", "GET")
    }

    /// Get real-time logs
    ///
    /// **Note**: This is a longhaul request, which will last forever until
    /// interrupted or disconnected.
    ///
    /// See [`longhaul_req`] for more information
    ///
    /// [`longhaul_req`]: Clash::longhaul_req
    pub fn get_log(&self) -> Result<LongHaul<Log>> {
        self.longhaul_req("logs", "GET")
    }

    /// Get specific proxy delay test information
    pub fn get_proxy_delay(&self, proxy: &str, test_url: &str, timeout: u64) -> Result<Delay> {
        use urlencoding::encode as e;
        let (proxy, test_url) = (e(proxy), e(test_url));
        self.get(&format!(
            "proxies/{}/delay?url={}&timeout={}",
            proxy, test_url, timeout
        ))
    }

    /// Select specific proxy
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
        Some(self.next_raw()?.and_then(Convert::convert))
    }

    pub fn next_raw(&mut self) -> Option<Result<String>> {
        let mut buf = String::with_capacity(30);
        match self.reader.read_line(&mut buf) {
            Ok(0) => None,
            Ok(_) => Some(Ok(buf)),
            Err(e) => Some(Err(Error::other(format!("{:}", e)))),
        }
    }
}

impl<T: DeserializeOwned> Iterator for LongHaul<T> {
    type Item = Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_item()
    }
}
