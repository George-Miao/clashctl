use std::fs::{File, OpenOptions};
use std::io::Read;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::time::Duration;
use std::{
    fmt::Display,
    io::{Seek, SeekFrom},
};

use log::{debug, info};
use ron::from_str;
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};
use url::Url;

use clashctl_core::{Clash, ClashBuilder};

use crate::{ConfigData, Error, Result};

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

#[derive(Debug)]
pub struct Config {
    inner: ConfigData,
    file: File,
}

// TODO: use config crate
impl Config {
    pub fn from_dir<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        debug!("Open config file @ {}", path.display());

        let mut this = if !path.exists() {
            info!("Config file not exist, creating new one");
            Self {
                inner: ConfigData::default(),
                file: File::create(path).map_err(Error::ConfigFileIoError)?,
            }
        } else {
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

            debug!("Raw config:\n{}", buf);

            let inner = from_str(&buf)?;

            drop(file);

            debug!("Content read");

            let file = File::create(path).map_err(Error::ConfigFileIoError)?;

            Self { inner, file }
        };

        this.write()?;
        Ok(this)
    }

    pub fn write(&mut self) -> Result<()> {
        let pretty_config = PrettyConfig::default().indentor("  ".to_owned());

        // Reset the file - Move cursor to 0 and truncate to 0
        self.file
            .seek(SeekFrom::Start(0))
            .and_then(|_| self.file.set_len(0))
            .map_err(Error::ConfigFileIoError)?;

        ron::ser::to_writer_pretty(&mut self.file, &self.inner, pretty_config)?;
        Ok(())
    }

    pub fn using_server(&self) -> Option<&Server> {
        match self.using {
            Some(ref using) => self.servers.iter().find(|x| &x.url == using),
            None => None,
        }
    }

    pub fn use_server(&mut self, url: Url) -> Result<()> {
        match self.get_server(&url) {
            Some(s) => {
                self.using = Some(url);
                Ok(())
            }
            None => Err(Error::ServerNotFound),
        }
    }

    pub fn get_server(&mut self, url: &Url) -> Option<&Server> {
        self.servers.iter().find(|x| &x.url == url)
    }

    pub fn get_inner(&self) -> &ConfigData {
        &self.inner
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
