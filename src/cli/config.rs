use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};
use std::time::Duration;

use log::debug;
use ron::from_str;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};

use crate::{Clash, ClashBuilder, Error, Result};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Server {
    pub url: url::Url,
    pub secret: Option<String>,
}

impl Server {
    pub fn into_clash_with_timeout(self, timeout: Option<Duration>) -> Result<Clash> {
        let mut builder = ClashBuilder::new(self.url)?;
        if let Some(secret) = self.secret {
            builder = builder.secret(secret)
        }
        if let Some(timeout) = timeout {
            builder = builder.timeout(timeout)
        }
        Ok(builder.build())
    }

    pub fn into_clash(self) -> Result<Clash> {
        self.into_clash_with_timeout(None)
    }
}

impl TryInto<Clash> for Server {
    type Error = Error;
    fn try_into(self) -> std::result::Result<Clash, Self::Error> {
        self.into_clash()
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
            Some(ref using) => self.servers.iter().find(|x| x.url.as_str() == using),
            None => None,
        }
    }

    pub fn use_server<'a, T: Into<&'a str>>(&mut self, url: T) -> Result<()> {
        let url = url.into();
        match self.servers.iter().find(|x| x.url.as_str() == url) {
            Some(s) => {
                self.using = Some(s.url.to_string());
                Ok(())
            }
            None => Err(Error::ServerNotFound),
        }
    }

    pub fn get_server<'a, T: Into<&'a str>>(&mut self, url: T) -> Option<&Server> {
        let url = url.into();
        self.servers.iter().find(|x| x.url.as_str() == url)
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
    pub using: Option<String>,
}

#[test]
fn test_config() {
    use log::Level;
    simple_logger::init_with_level(Level::Debug).unwrap();
    let mut config = Config::from_dir("/tmp/test.ron").unwrap();
    config.write().unwrap();
    config.servers.push(Server {
        url: url::Url::parse("http://proxy.lan:9090").unwrap(),
        secret: None,
    });
    config.write().unwrap();
}
