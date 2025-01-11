mod cli;
mod commands;
mod config;
mod destinations;
mod errors;

use clap::Parser;
use cli::{Cli, Command, Destinations};
use commands::{bluesky, linkedin, turso};
use errors::MusketError;

/// Runs the main logic of the application.
///
/// # Errors
///
/// This function will return an error if any of the commands fail.
pub async fn run() -> Result<Vec<String>, MusketError> {
    let mut success_messages: Vec<String> = vec![];
    let cli = Cli::parse();

    match cli.cmd {
        Command::Init { force } => {
            let overwrite = force.unwrap_or(false);

            if config::configuration_exists()? && !overwrite {
                return Err(MusketError::Cli {
                    message: "The configuration file already exists. If you want to overwrite it, please run musket init command with the -f, --force option.".to_string(),
                });
            }

            match config::configure() {
                Ok(_) => {
                    success_messages.push(format!("The configuration file has been created here: \"{}\". \nTo start using Musket, please fill in the configuration file with your data.",
                    config::get_configuration_path()
                        .unwrap_or_default()
                        .to_string_lossy()));
                }
                Err(e) => return Err(e.into()),
            }
        }
        Command::Fire {
            url,
            destination,
            tags,
            commentary,
        } => {
            if destination.is_none() {
                return Err(MusketError::Cli {
                    message: format!("The url \"{url}\" cannot be sent to a non-existing destination. Set, at least, one valid destination."),
                });
            }

            let cfg = config::configure()?;

            let vector_of_tags = tags.unwrap_or_default();
            let destinations = destination.unwrap_or_default();

            for target in destinations {
                match target {
                    Destinations::All => {
                        success_messages.push(
                            bluesky::execute(&cfg, &url, &vector_of_tags, commentary.as_ref())
                                .await?,
                        );
                        success_messages.push(
                            linkedin::execute(&cfg, &url, &vector_of_tags, commentary.as_ref())
                                .await?,
                        );
                        success_messages.push(turso::execute(&cfg, &url, &vector_of_tags).await?);
                    }
                    Destinations::Bluesky => {
                        success_messages.push(
                            bluesky::execute(&cfg, &url, &vector_of_tags, commentary.as_ref())
                                .await?,
                        );
                    }
                    Destinations::LinkedIn => {
                        success_messages.push(
                            linkedin::execute(&cfg, &url, &vector_of_tags, commentary.as_ref())
                                .await?,
                        );
                    }
                    Destinations::Turso => {
                        success_messages.push(turso::execute(&cfg, &url, &vector_of_tags).await?);
                    }
                }
            }
        }
    }
    Ok(success_messages)
}
