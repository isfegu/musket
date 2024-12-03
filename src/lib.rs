mod cli;
mod commands;
mod config;
mod destinations;
mod errors;

use clap::Parser;
use cli::{Cli, Command, Destinations};
use commands::*;
use errors::*;

pub async fn run() -> Result<(), MusketError> {
    let cli = Cli::parse();

    match cli.cmd {
        Command::Init => match config::configure() {
            Ok(_) => {
                println!(
                    "The configuration file has been created here: \"{}\". \nTo start using Musket, please fill in the configuration file with your data.",
                    config::get_configuration_path()
                        .unwrap_or_default()
                        .to_string_lossy()
                );
            }
            Err(e) => {
                return Err(e.into());
            }
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

            for target in destinations {
                match target {
                    Destinations::All => {
                        turso::execute(&cfg, &url, &vector_of_tags).await?;
                        linkedin::execute(&cfg, &url, &vector_of_tags).await?;
                    }
                    Destinations::Turso => {
                        turso::execute(&cfg, &url, &vector_of_tags).await?;
                    }
                    Destinations::LinkedIn => {
                        linkedin::execute(&cfg, &url, &vector_of_tags).await?;
                    }
                }
            }
        }
    }
    Ok(())
}
