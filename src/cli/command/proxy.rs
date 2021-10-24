use std::time::Duration;

use clap::Subcommand;

use log::{error, info};
use owo_colors::OwoColorize;
use requestty::{prompt_one, Answer, ListItem, Question};

use crate::cli::{Flags, ProxySort};
use crate::{Clash, Result};

#[derive(Subcommand, Debug)]
#[clap(about = "Interacting with proxies")]
pub enum ProxySubcommand {
    #[clap(alias = "ls", about = "List proxies (alias ls)")]
    List,
    #[clap(about = "Set proxies")]
    Set,
}

impl ProxySubcommand {
    pub async fn handle(&self, flags: &Flags) -> Result<()> {
        match self {
            ProxySubcommand::List => {
                let server = "http://proxy.lan:9090";
                let clash = Clash::builder(server)?
                    .timeout(Some(Duration::from_millis(flags.timeout)))
                    .build();
                let proxies = clash.get_proxies().await?;
                proxies.render_list(ProxySort::by_delay())?;
            }
            ProxySubcommand::Set => {
                let server = "http://proxy.lan:9090";
                let clash = Clash::builder(server)?
                    .timeout(Some(Duration::from_millis(flags.timeout)))
                    .build();
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
                    Ok(result) => match result {
                        Answer::ListItem(ListItem { text, .. }) => text,
                        _ => {
                            unreachable!()
                        }
                    },
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
