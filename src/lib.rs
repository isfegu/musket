mod cli;
mod commands;
mod config;
mod destinations;
mod errors;
mod tui;

use std::str::FromStr;

use clap::Parser;
use cli::{Cli, Command, Destinations};
use commands::*;
use config::Configuration;
use errors::*;

pub async fn run() -> Result<Vec<String>, MusketError> {
    let mut success_messages: Vec<String> = vec![];
    let cli = Cli::parse();

    match cli.cmd {
        Command::Init => match config::configure() {
            Ok(_) => {
                success_messages.push(format!("The configuration file has been created here: \"{}\". \nTo start using Musket, please fill in the configuration file with your data.",
                config::get_configuration_path()
                    .unwrap_or_default()
                    .to_string_lossy()));
            }
            Err(e) => return Err(e.into()),
        },
        Command::Fire {
            url,
            destination,
            tags,
        } => {
            if destination.is_none() {
                return Err(MusketError::Cli {
                    message: format!("The url \"{}\" cannot be sent to a non-existing destination. Set, at least, one valid destination.", url),
                });
            }

            let cfg = config::configure()?;

            let vector_of_tags = tags.unwrap_or_default();
            let destinations = destination.unwrap_or_default();

            success_messages = publish(url, destinations, vector_of_tags, cfg).await?;
        }
        Command::Load => {
            let pack = tui::main().await?;

            if let Some(pack) = pack {
                let cfg = config::configure()?;
                let destinations = pack
                    .destinations
                    .into_iter()
                    .map(|s| FromStr::from_str(&s))
                    .collect::<Result<Vec<Destinations>, _>>()?;
                let message = pack.message.unwrap_or_default().join("\n");
                // TODO: Add tags to the TUI
                let tags = vec![];
                success_messages = publish(message, destinations, tags, cfg).await?;
            }
        }
    }
    Ok(success_messages)
}

async fn publish(
    message: String,
    destinations: Vec<Destinations>,
    tags: Vec<String>,
    cfg: Configuration,
) -> Result<Vec<String>, MusketError> {
    let mut responses = vec![];

    for target in destinations {
        match target {
            Destinations::All => {
                responses.push(bluesky::execute(&cfg, &message, &tags).await?);
                responses.push(linkedin::execute(&cfg, &message, &tags).await?);
                responses.push(turso::execute(&cfg, &message, &tags).await?);
            }
            Destinations::Bluesky => {
                responses.push(bluesky::execute(&cfg, &message, &tags).await?);
            }
            Destinations::LinkedIn => {
                responses.push(linkedin::execute(&cfg, &message, &tags).await?);
            }
            Destinations::Turso => {
                responses.push(turso::execute(&cfg, &message, &tags).await?);
            }
        }
    }

    Ok(responses)
}
