use std::path::PathBuf;
use std::time::Duration;

use clap::Parser;
use clashctl_core::Clash;
use home::home_dir;
use log::debug;
use url::Url;

use crate::{Config, Error, Result};

const DEFAULT_TEST_URL: &str = "http://www.gstatic.com/generate_204";

#[derive(Clone, Debug, Parser)]
pub struct Flags {
    #[clap(short, long, parse(from_occurrences))]
    /// Verbosity. Default: INFO, -v DEBUG, -vv TRACE
    pub verbose: u8,

    #[clap(short, long, default_value = "2000")]
    /// Timeout of requests, in ms
    pub timeout: u64,

    #[clap(long, conflicts_with = "config-path")]
    /// Path of config directory. Default to ~/.config/clashctl
    pub config_dir: Option<PathBuf>,

    #[clap(short, long, conflicts_with = "config-dir")]
    /// Path of config file. Default to ~/.config/clashctl/config.ron
    pub config_path: Option<PathBuf>,

    #[clap(
            long,
            default_value = DEFAULT_TEST_URL,
        )
    ]
    /// Url for testing proxy endpointes
    pub test_url: Url,
}

impl Default for Flags {
    fn default() -> Self {
        Self {
            verbose: 0,
            timeout: 2000,
            config_dir: None,
            config_path: None,
            test_url: Url::parse(DEFAULT_TEST_URL).unwrap(),
        }
    }
}

impl Flags {
    pub fn get_config(&self) -> Result<Config> {
        if let Some(ref dir) = self.config_path {
            return Config::from_dir(dir);
        }
        let conf_dir = self
            .config_dir
            .to_owned()
            .or_else(|| home_dir().map(|dir| dir.join(".config/clashctl/")))
            .ok_or(Error::ConfigFileOpenError)?;

        if !conf_dir.exists() {
            debug!("Config directory does not exist, creating.");
            std::fs::create_dir_all(&conf_dir).map_err(Error::ConfigFileIoError)?;
        }

        if !conf_dir.is_dir() {
            Err(Error::ConfigFileTypeError(conf_dir))
        } else {
            debug!("Path to config: {}", conf_dir.display());
            Config::from_dir(conf_dir.join("config.ron"))
        }
    }

    pub fn connect_server_from_config(&self) -> Result<Clash> {
        let config = self.get_config()?;
        let server = config
            .using_server()
            .ok_or(Error::ServerNotFound)?
            .to_owned();
        server.into_clash_with_timeout(Some(Duration::from_millis(self.timeout)))
    }
}
