use clap::Subcommand;

use log::{debug, info, warn};
use owo_colors::OwoColorize;
use requestty::{prompt, prompt_one, Question};
use terminal_size::{terminal_size, Height, Width};
use url::Url;

use crate::cli::{Flags, Server};
use crate::Result;

#[derive(Subcommand, Debug)]
#[clap(about = "Interacting with servers")]
pub enum ServerSubcommand {
    #[clap(alias = "a", about = "Add new server (alias a)")]
    Add,
    #[clap(about = "Select active server")]
    Select,
    #[clap(alias = "ls", about = "Show current active server")]
    List,
}

impl ServerSubcommand {
    pub async fn handle(&self, flags: &Flags) -> Result<()> {
        let mut config = flags.get_config()?;

        match self {
            ServerSubcommand::Add => {
                let questions = [
                    Question::input("url")
                        .message("URL of Clash API")
                        .validate(|input, _| match Url::parse(input) {
                            Ok(_) => Ok(()),
                            Err(e) => Err(format!("Invalid URL: {}", e)),
                        })
                        .build(),
                    Question::password("secret")
                        .message("Secret of Clash API, default to None:")
                        .build(),
                ];
                let mut res = prompt(questions).expect("Error during prompt");
                debug!("{:#?}", res);
                let secret = match res.remove("secret").unwrap().try_into_string().unwrap() {
                    string if string == "".to_owned() => None,
                    secret => Some(secret),
                };

                let url_str = res.remove("url").unwrap().try_into_string().unwrap();
                let url = Url::parse(&url_str).unwrap();

                let server = Server {
                    secret: secret,
                    url: url.clone(),
                };

                info!("Adding {}", server);

                config.servers.push(server);
                debug!("{:#?}", config.servers);
                config.use_server(url)?;
                config.write()?;
            }
            ServerSubcommand::Select => {
                if config.servers.len() == 0 {
                    warn!("No server configured yet. Use `clashctl server add` first.");
                    return Ok(());
                }
                let servers = config.servers.iter().map(|server| server.url.as_str());
                let ans = &prompt_one(
                    Question::select("server")
                        .message("Select active server to interact with")
                        .choices(servers)
                        .build(),
                )?;
                let ans_str = &ans.as_list_item().unwrap().text;
                config.use_server(Url::parse(ans_str).unwrap())?;
                config.write()?;
            }
            ServerSubcommand::List => {
                if config.servers.len() == 0 {
                    warn!("No server configured yet. Use `clashctl server add` first.");
                    return Ok(());
                }
                let (Width(terminal_width), _) = terminal_size().unwrap_or((Width(70), Height(0)));
                let active = config.using_server();
                println!("\n{:-<1$}", "", terminal_width as usize);
                println!("{:<8}{:<50}", "ACTIVE".green(), "URL");
                println!("{:-<1$}", "", terminal_width as usize);
                for server in &config.servers {
                    let is_active = match active {
                        Some(active) => server == active,
                        _ => false,
                    };
                    println!(
                        "{:^8}{:<50}",
                        if is_active { ">".green() } else { "".green() },
                        server.url.as_str(),
                    )
                }
                println!("{:-<1$}\n", "", terminal_width as usize);
            }
        }
        Ok(())
    }
}
