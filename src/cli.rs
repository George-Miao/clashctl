use clap::{Parser, Subcommand};
use clap_generate::Shell;
use requestty::{prompt_one, Answer, ListItem, Question};

use crate::{Clash, Result};

#[derive(Parser, Debug)]
#[clap(
    version = "0.0.1",
    author = "George Miao <gm@miao.dev>",
    about = "CLI used to interact with Clash"
)]
pub struct Opts {
    #[clap(subcommand)]
    pub cmd: Cmd,
    #[clap(flatten)]
    pub config: Config,
}

#[derive(Subcommand, Debug)]
pub enum Cmd {
    #[clap(subcommand)]
    Proxy(ProxySubcommand),
    #[clap(subcommand)]
    Server(ServerSubcommand),
    Completion(CompletionArg),
}

#[derive(Subcommand, Debug)]
#[clap(about = "Interacting with proxies")]
pub enum ProxySubcommand {
    List,
    Set,
}

impl ProxySubcommand {
    pub async fn handle(&self) -> Result<()> {
        match self {
            ProxySubcommand::List => {
                let server = "http://proxy.lan:9090";
                let clash = Clash::builder(server)?.build();
                let proxies = clash.get_proxies().await?;
                println!("{:<40}, {}", "NAME", "TYPE");
                for (name, proxy) in proxies.iter() {
                    println!("{:<40}, {:?}", name, proxy.proxy_type)
                }
            }
            ProxySubcommand::Set => {
                let server = "http://proxy.lan:9090";
                let clash = Clash::builder(server)?.build();
                let proxies = clash.get_proxies().await?;
                let mut groups = proxies
                    .iter()
                    .filter(|(_, p)| p.proxy_type.is_selector())
                    .map(|(name, _)| name)
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
                        eprintln!("Error selecting proxy: {}", e);
                        return Err(e.into());
                    }
                };
                let proxy = clash.get_proxy(&group_selected).await?;
                if let Some(members) = proxy.all {
                    let member_selected = match prompt_one(
                        Question::select("proxy")
                            .message("Which proxy to use?")
                            .choices(members)
                            .build(),
                    ) {
                        Ok(result) => match result {
                            Answer::ListItem(ListItem { text, .. }) => text,
                            _ => {
                                unreachable!()
                            }
                        },
                        Err(e) => {
                            eprintln!("Error selecting proxy: {}", e);
                            return Err(e.into());
                        }
                    };
                    println!("Selected member {} of {}", member_selected, group_selected);
                } else {
                    eprintln!("Empty group")
                }
            }
        }
        Ok(())
    }
}

#[derive(Subcommand, Debug)]
#[clap(about = "Interacting with servers")]
pub enum ServerSubcommand {
    List,
    Select,
    Default,
}

impl ServerSubcommand {
    pub async fn handle(&self) -> Result<()> {
        match self {
            ServerSubcommand::List => {}
            ServerSubcommand::Select => {}
            ServerSubcommand::Default => {}
        }
        Ok(())
    }
}

#[derive(Parser, Debug)]
#[clap(about = "Generate auto-completion scripts")]
pub struct CompletionArg {
    #[clap(possible_values=&[
        "bash",
        "elvish",
        "fish",
        "powershell",
        "zsh",
    ])]
    pub shell: Shell,
}

#[derive(Parser, Debug)]
pub struct Config {}
