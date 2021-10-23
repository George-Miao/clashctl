use clap::Subcommand;

use log::info;
use requestty::{prompt, Question};

use crate::cli::Flags;
use crate::Result;

#[derive(Subcommand, Debug)]
#[clap(about = "Interacting with servers")]
pub enum ServerSubcommand {
    #[clap(alias = "a", about = "Add new server (alias a)")]
    Add,
    #[clap(alias = "ls", about = "List servers comfigured (alias ls)")]
    List,
    #[clap(about = "Select active server")]
    Select,
}

impl ServerSubcommand {
    pub async fn handle(&self, flags: &Flags) -> Result<()> {
        match self {
            ServerSubcommand::Add => {
                let questions = [
                    Question::input("url")
                        .message("URL of Clash API")
                        .validate(|input, _| match url::Url::parse(input) {
                            Ok(_) => Ok(()),
                            Err(e) => Err(format!("Invalid URL: {}", e)),
                        })
                        .build(),
                    Question::password("secret")
                        .message("Secret of Clash API, default to None:")
                        .build(),
                ];
                let res = prompt(questions).expect("Error during prompt");
                info!("{:#?}", res);
            }
            ServerSubcommand::List => {}
            ServerSubcommand::Select => {}
        }
        Ok(())
    }
}
