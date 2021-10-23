use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use clap::{ArgEnum, IntoApp, Parser, Subcommand};
use clap_generate::{generate, Shell};
use log::{error, info, warn};
use owo_colors::OwoColorize;
use requestty::{prompt_one, Answer, ListItem, Question};

use crate::cli::{detect_shell, ProxySort};
use crate::{Clash, Result};

#[derive(Parser, Debug)]
#[clap(
    version = "0.1.0",
    author = "George Miao <gm@miao.dev>",
    about = "CLI used to interact with Clash RESTful API"
)]
pub struct Opts {
    #[clap(subcommand)]
    pub cmd: Cmd,
    #[clap(flatten)]
    pub config: Config,
}

#[derive(Parser, Debug)]
pub struct Config {}

#[derive(Subcommand, Debug)]
pub enum Cmd {
    #[clap(subcommand)]
    Proxy(ProxySubcommand),
    #[clap(subcommand)]
    Server(ServerSubcommand),
    #[clap(alias = "comp")]
    Completion(CompletionArg),
}

#[derive(Subcommand, Debug)]
#[clap(about = "Interacting with proxies")]
pub enum ProxySubcommand {
    #[clap(alias = "ls", about = "List proxies (alias ls)")]
    List,
    #[clap(alias = "ls", about = "Set proxies")]
    Set,
}

#[derive(Subcommand, Debug)]
#[clap(about = "Interacting with servers")]
pub enum ServerSubcommand {
    #[clap(alias = "a", about = "Add new server (alias a)")]
    Add,
    #[clap(alias = "ls", about = "List servers comfigured (alias ls)")]
    List,
    Select,
    Default,
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
    pub shell: Option<Shell>,
    #[clap(
        short,
        long,
        about = "Output completion script to file, default to STDOUT"
    )]
    pub output: Option<PathBuf>,
}

impl ProxySubcommand {
    pub async fn handle(&self) -> Result<()> {
        match self {
            ProxySubcommand::List => {
                let server = "http://proxy.lan:9090";
                let clash = Clash::builder(server)?.build();
                let proxies = clash.get_proxies().await?;
                proxies.render_list(ProxySort::by_delay())?;
            }
            ProxySubcommand::Set => {
                let server = "http://proxy.lan:9090";
                let clash = Clash::builder(server)?.build();
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

impl ServerSubcommand {
    pub async fn handle(&self) -> Result<()> {
        match self {
            ServerSubcommand::Add => {}
            ServerSubcommand::List => {}
            ServerSubcommand::Select => {}
            ServerSubcommand::Default => {}
        }
        Ok(())
    }
}

impl CompletionArg {
    pub fn handle(&self) -> Result<()> {
        match self.shell.or_else(detect_shell) {
            None => {
                warn!("Shell not detected or it's not supported");
                warn!(
                    "Supported shells: {}",
                    Shell::value_variants()
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Some(shell) => {
                // let mut out: Box<dyn Write> = self
                //     .output
                //     .and_then(|x| File::open(x).ok())
                //     .or_else(|| std::io::stdout())
                //     .unwrap();
                let mut out: Box<dyn Write> = match self.output {
                    Some(ref dir) => Box::new(
                        File::create(&dir).expect(&format!("Unable to open {}", dir.display())),
                    ),
                    None => Box::new(std::io::stdout()),
                };
                generate(shell, &mut Opts::into_app(), "clashctl", &mut out);
            }
        }
        Ok(())
    }
}
