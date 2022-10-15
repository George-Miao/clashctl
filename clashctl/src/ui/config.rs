use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use once_cell::sync::OnceCell;
use smart_default::SmartDefault;

use crate::interactive::{clashctl::model::Config as ConfigModel, Config, ConfigData};

// static CONFIG: OnceCell<RwLock<Config>> = OnceCell::new();
static CONFIG: OnceCell<RwLock<Config>> = OnceCell::new();

static NONE: &str = "N/A";

pub fn init_config(config: Config) {
    let _ = CONFIG.set(RwLock::new(config));
}

pub fn get_config<'a>() -> RwLockReadGuard<'a, Config> {
    CONFIG
        .get()
        .expect("Config is not initialized")
        .read()
        .unwrap()
}

pub fn get_config_mut<'a>() -> RwLockWriteGuard<'a, Config> {
    CONFIG
        .get()
        .expect("Config is not initialized")
        .write()
        .unwrap()
}

#[derive(Clone, Debug, SmartDefault)]
pub struct ConfigState {
    clash: Option<ConfigModel>,
    #[default(_code = "{ get_config().get_inner().clone() }")]
    clashctl: ConfigData,
    // offset: usize,
}

impl ConfigState {
    pub fn clashctl_list(&self) -> impl Iterator<Item = (&str, String)> {
        let server = self
            .clashctl
            .using
            .as_ref()
            .map(|x| x.to_string())
            .unwrap_or_else(|| "N/A".to_owned());
        let log_dir = self
            .clashctl
            .tui
            .log_file
            .as_ref()
            .and_then(|x| x.to_str())
            .unwrap_or("N/A")
            .to_string();
        [("Server", server), ("Log dir", log_dir)].into_iter()
    }

    pub fn clash_list(&self) -> impl Iterator<Item = (&str, String)> {
        match self.clash {
            Some(ref conf) => vec![
                ("Port", conf.port.to_string()),
                ("Socks Port", conf.socks_port.to_string()),
                ("Redir Port", conf.redir_port.to_string()),
                ("Tproxy Port", conf.tproxy_port.to_string()),
                ("Mixed Port", conf.mixed_port.to_string()),
                ("Allow Lan", conf.allow_lan.to_string()),
                ("Ipv6", conf.ipv6.to_string()),
                ("Mode", conf.mode.to_string()),
                ("Log Level", conf.log_level.to_string()),
                ("Bind_Address", conf.bind_address.to_string()),
                ("Authentication", conf.authentication.len().to_string()),
            ]
            .into_iter(),
            None => vec![
                ("Port", NONE.into()),
                ("Socks Port", NONE.into()),
                ("Redir Port", NONE.into()),
                ("Tproxy Port", NONE.into()),
                ("Mixed Port", NONE.into()),
                ("Allow Lan", NONE.into()),
                ("Ipv6", NONE.into()),
                ("Mode", NONE.into()),
                ("Log Level", NONE.into()),
                ("Bind Address", NONE.into()),
                ("Authentication", NONE.into()),
            ]
            .into_iter(),
        }
    }

    pub fn update_clash(&mut self, config: ConfigModel) {
        self.clash = Some(config)
    }
}
