use std::fmt::Display;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};
use std::time::Duration;

use log::debug;
use ron::from_str;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{Clash, ClashBuilder, Error, Result};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Server {
    pub url: url::Url,
    pub secret: Option<String>,
}

impl Server {
    pub fn into_clash_with_timeout(self, timeout: Option<Duration>) -> Result<Clash> {
        Ok(self.into_clash_builder()?.timeout(timeout).build())
    }

    pub fn into_clash(self) -> Result<Clash> {
        self.into_clash_with_timeout(None)
    }

    pub fn into_clash_builder(self) -> Result<ClashBuilder> {
        Ok(ClashBuilder::new(self.url)?.secret(self.secret))
    }
}

impl Display for Server {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Server ({})", self.url)
    }
}

impl TryInto<Clash> for Server {
    type Error = Error;
    fn try_into(self) -> std::result::Result<Clash, Self::Error> {
        self.into_clash()
    }
}

impl TryInto<ClashBuilder> for Server {
    type Error = Error;
    fn try_into(self) -> std::result::Result<ClashBuilder, Self::Error> {
        self.into_clash_builder()
    }
}

pub struct Config {
    inner: ConfigData,
    path: PathBuf,
}

impl Config {
    pub fn from_dir<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        debug!("Open config file @ {}", path.display());

        if !path.exists() {
            debug!("File not exist, creating new one");
            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .open(path)
                .map_err(Error::ConfigFileIoError)?;
            let default_conf = to_string_pretty(&ConfigData::default(), Default::default())?;
            file.write(default_conf.as_bytes())
                .map_err(Error::ConfigFileIoError)?;
        }

        debug!("Reading and parsing config file");

        let mut file = OpenOptions::new()
            .read(true)
            .open(path)
            .map_err(Error::ConfigFileIoError)?;

        let mut buf = match file.metadata() {
            Ok(meta) => String::with_capacity(meta.len() as usize),
            Err(_) => String::new(),
        };

        file.read_to_string(&mut buf)
            .map_err(Error::ConfigFileIoError)?;

        debug!("Content read");

        let data = from_str::<ConfigData>(&buf)?;

        Ok(Self {
            inner: data,
            path: path.to_owned(),
        })
    }

    pub fn write(&self) -> Result<()> {
        let formatted = to_string_pretty(&self.inner, PrettyConfig::new())?;
        let mut file = File::create(&self.path).map_err(Error::ConfigFileIoError)?;
        file.write_all(formatted.as_bytes())
            .map_err(Error::ConfigFileIoError)?;
        Ok(())
    }

    pub fn using_server(&self) -> Option<&Server> {
        match self.using {
            Some(ref using) => self.servers.iter().find(|x| &x.url == using),
            None => None,
        }
    }

    pub fn use_server(&mut self, url: Url) -> Result<()> {
        match self.get_server(url) {
            Some(s) => {
                self.using = Some(s.url.clone());
                Ok(())
            }
            None => Err(Error::ServerNotFound),
        }
    }

    pub fn get_server(&mut self, url: Url) -> Option<&Server> {
        self.servers.iter().find(|x| x.url == url)
    }
}

impl Deref for Config {
    type Target = ConfigData;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Config {
    fn deref_mut(&mut self) -> &mut ConfigData {
        &mut self.inner
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ConfigData {
    pub servers: Vec<Server>,
    pub using: Option<Url>,
}

#[test]
fn test_config() {
    use log::Level;
    use std::env;
    simple_logger::init_with_level(Level::Debug).unwrap();
    let mut config = Config::from_dir("/tmp/test.ron").unwrap();
    config.write().unwrap();
    config.servers.push(Server {
        url: url::Url::parse(&env::var("PROXY_ADDR").unwrap()).unwrap(),
        secret: None,
    });
    config.write().unwrap();
}
