use std::str::FromStr;
use std::time::Duration;

use clap::{Parser, Subcommand};

use log::{error, info, warn};
use owo_colors::OwoColorize;
use requestty::{prompt_one, Answer, ListItem, Question};

use crate::cli::{Flags, ProxySort};
use crate::{Error, Result};
use crate::model::ProxyType;


impl FromStr for ProxyType {
    type Err = Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "direct" => Ok(Self::Direct),
            "reject" => Ok(Self::Reject),
            "selector" => Ok(Self::Selector),
            "urltest" => Ok(Self::URLTest),
            "fallback" => Ok(Self::Fallback),
            "loadbalance" => Ok(Self::LoadBalance),
            "shadowsocks" => Ok(Self::Shadowsocks),
            "vmess" => Ok(Self::Vmess),
            "ssr" => Ok(Self::ShadowsocksR),
            "http" => Ok(Self::Http),
            "snell" => Ok(Self::Snell),
            "trojan" => Ok(Self::Trojan),
            "socks5" => Ok(Self::Socks5),
            "relay" => Ok(Self::Relay),
            _ => Err(Error::BadOption)
        }
    }
}

#[derive(Subcommand, Debug)]
#[clap(about = "Interacting with proxies")]
pub enum ProxySubcommand {
    #[clap(alias = "ls", about = "List proxies (alias ls)")]
    List(ProxyListOpt),
    #[clap(about = "Set active proxy")]
    Use,
}

#[derive(Parser, Debug, Clone)]
pub struct ProxyListOpt {
    #[clap(
        short, 
        long, 
        default_value = "type", 
        possible_values = &["type", "name", "delay"],
    )]
    pub sort: ProxySort,
    #[clap(short, long, about = "Reverse the listed result")]
    pub reverse: bool,
    #[clap(
        short,
        long,
        about = "Exclude proxy types",
        name = "PROXY_TYPES",
        possible_values = &[
            "direct",
            "reject",
            "selector",
            "urltest",
            "fallback",
            "loadbalance",
            "shadowsocks",
            "vmess",
            "ssr",
            "http",
            "snell",
            "trojan",
            "socks5",
            "relay"
        ],
    )]
    pub exclude: Vec<ProxyType>,
    #[clap(
        short = 't', 
        long = "type", 
        about = "Only show selected proxy types",
        possible_values = &[
            "direct",
            "reject",
            "selector",
            "urltest",
            "fallback",
            "loadbalance",
            "shadowsocks",
            "vmess",
            "ssr",
            "http",
            "snell",
            "trojan",
            "socks5",
            "relay"
        ],
    )]
    pub proxy_types: Vec<ProxyType>
}

impl ProxySubcommand {
    pub async fn handle(&self, flags: &Flags) -> Result<()> {
        let config = flags.get_config()?;
        let server = match config.using_server() {
            Some(server) => server.to_owned(),
            None => {
                warn!("No server configured yet. Use `clashctl server add` first.");
                return Ok(());
            }
        };
        info!("Using {}", server);
        let clash = server.into_clash_with_timeout(Some(Duration::from_millis(flags.timeout)))?;

        match self {
            ProxySubcommand::List(opt) => {
                let proxies = clash.get_proxies().await?;
                proxies.render_list(opt)?;
            }
            ProxySubcommand::Use => {
                let proxies = clash.get_proxies().await?;
                let mut groups = proxies
                    .iter()
                    .filter(|(_, p)| p.proxy_type.is_selector())
                    .map(|(name, _)| name)
                    .filter(|name| !["GLOBAL", "REJECT"].contains(&name.as_str()))
                    .collect::<Vec<_>>();
                groups.sort();
                let group_selected = match prompt_one(
                    Question::select("proxy")
                        .message("Which group to change?")
                        .choices(groups)
                        .build(),
                ) {
                    Ok(result) => result.as_list_item().unwrap().text.to_owned(),
                    Err(e) => {
                        error!("Error selecting proxy: {}", e);
                        return Err(e.into());
                    }
                };
                let proxy = clash.get_proxy(&group_selected).await?;

                // all / now only occurs when proxy_type is [`ProxyType::Selector`]
                let members = proxy.all.unwrap();
                let now = proxy.now.unwrap();
                let cur_index = members.iter().position(|x| x == &now).unwrap();
                let mut question = Question::select("proxy")
                    .message("Which proxy to use?")
                    .choices(members);
                if cur_index != 0 {
                    question = question.default(cur_index)
                }
                let member_selected = match prompt_one(question.build()) {
                    Ok(result) => match result {
                        Answer::ListItem(ListItem { text, .. }) => text,
                        _ => unreachable!(),
                    },
                    Err(e) => {
                        error!("Error selecting proxy: {}", e);
                        return Err(e.into());
                    }
                };
                info!(
                    "Setting group {} to use {}",
                    group_selected.green(),
                    member_selected.green()
                );
                clash
                    .set_proxygroup_selected(&group_selected, &member_selected)
                    .await?;
                info!("Done!")
            }
        }
        Ok(())
    }
}

#[test]
fn test_proxy_type() {
    let string = "direct";
    let parsed = string.parse().unwrap();
    assert_eq!(ProxyType::Direct, parsed);
}