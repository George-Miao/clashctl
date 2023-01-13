use std::{
    fmt::Display,
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    ops::{Deref, DerefMut},
    path::Path,
    time::Duration,
};

use clashctl_core::{Clash, ClashBuilder};
use log::{debug, info};
use ron::{from_str, ser::PrettyConfig};
use serde::{Deserialize, Serialize};
use url::Url;

use super::{ConfigData, InteractiveError, InteractiveResult};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Server {
    pub url: url::Url,
    pub secret: Option<String>,
}

impl Server {
    pub fn into_clash_with_timeout(self, timeout: Option<Duration>) -> InteractiveResult<Clash> {
        Ok(self.into_clash_builder()?.timeout(timeout).build())
    }

    pub fn into_clash(self) -> InteractiveResult<Clash> {
        self.into_clash_with_timeout(None)
    }

    pub fn into_clash_builder(self) -> InteractiveResult<ClashBuilder> {
        Ok(ClashBuilder::new(self.url)?.secret(self.secret))
    }
}

impl Display for Server {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Server ({})", self.url)
    }
}

impl TryInto<Clash> for Server {
    type Error = InteractiveError;

    fn try_into(self) -> std::result::Result<Clash, Self::Error> {
        self.into_clash()
    }
}

impl TryInto<ClashBuilder> for Server {
    type Error = InteractiveError;

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
    pub fn from_dir<P: AsRef<Path>>(path: P) -> InteractiveResult<Self> {
        let path = path.as_ref();

        debug!("Open config file @ {}", path.display());

        let mut this = if !path.exists() {
            info!("Config file not exist, creating new one");
            Self {
                inner: ConfigData::default(),
                file: File::create(path).map_err(InteractiveError::ConfigFileIoError)?,
            }
        } else {
            debug!("Reading and parsing config file");

            let mut file = OpenOptions::new()
                .read(true)
                .open(path)
                .map_err(InteractiveError::ConfigFileIoError)?;

            let mut buf = match file.metadata() {
                Ok(meta) => String::with_capacity(meta.len() as usize),
                Err(_) => String::new(),
            };

            file.read_to_string(&mut buf)
                .map_err(InteractiveError::ConfigFileIoError)?;

            debug!("Raw config:\n{}", buf);

            let inner = from_str(&buf)?;

            drop(file);

            debug!("Content read");

            let file = File::create(path).map_err(InteractiveError::ConfigFileIoError)?;

            Self { inner, file }
        };

        this.write()?;
        Ok(this)
    }

    pub fn write(&mut self) -> InteractiveResult<()> {
        let pretty_config = PrettyConfig::default().indentor("  ".to_owned());

        // Reset the file - Move cursor to 0 and truncate to 0
        self.file
            .seek(SeekFrom::Start(0))
            .and_then(|_| self.file.set_len(0))
            .map_err(InteractiveError::ConfigFileIoError)?;

        ron::ser::to_writer_pretty(&mut self.file, &self.inner, pretty_config)?;
        self.file
            .flush()
            .map_err(InteractiveError::ConfigFileIoError)?;

        Ok(())
    }

    pub fn using_server(&self) -> Option<&Server> {
        match self.using {
            Some(ref using) => self.servers.iter().find(|x| &x.url == using),
            None => None,
        }
    }

    pub fn use_server(&mut self, url: Url) -> InteractiveResult<()> {
        match self.get_server(&url) {
            Some(_s) => {
                self.using = Some(url);
                Ok(())
            }
            None => Err(InteractiveError::ServerNotFound),
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
    use std::env;

    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let mut config = Config::from_dir("/tmp/test.ron").unwrap();
    config.write().unwrap();
    config.servers.push(Server {
        url: url::Url::parse(&env::var("PROXY_ADDR").unwrap()).unwrap(),
        secret: None,
    });
    config.write().unwrap();
}
