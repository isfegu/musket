mod cli;
mod config;
mod destinations;
mod errors;
mod shooters;

use clap::Parser;
use cli::{Cli, Command};
use destinations::Destinations;
use errors::MusketError;
use shooters::{bluesky_shooter, linkedin_shooter, mastodon_shooter, turso_shooter};
use tracing::debug;
use tracing_subscriber::EnvFilter;

/// Runs the main logic of the application.
///
/// # Errors
///
/// This function will return an error if any of the commands fail.
pub async fn run() -> Result<Vec<String>, MusketError> {
    let mut success_messages: Vec<String> = vec![];
    let cli = Cli::parse();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .without_time()
        .init();

    match cli.cmd {
        Command::Init { force } => {
            debug!("Inside run function. Command::Init");

            let overwrite = force.unwrap_or(false);

            if config::configuration_exists()? && !overwrite {
                return Err(MusketError::Cli {
                    message: "The configuration file already exists. If you want to overwrite it, please run the musket init command with the -f, --force option.".to_string(),
                });
            }

            match config::configure() {
                Ok(_) => {
                    success_messages.push(format!("The configuration file has been created here: \"{}\". \nTo start using Musket, please complete the configuration file with your data.",
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
            debug!("Inside run function. Command::Fire");

            if !config::configuration_exists()? {
                return Err(MusketError::Cli {
                    message: format!("The configuration file does not exist. If you want to send \"{url}\" to {} destination, please first run the musket init command and next fill the configuration file.", destination.unwrap_or_default().iter().map(std::string::ToString::to_string).collect::<Vec<String>>().join(", ")),
                });
            }

            if destination.is_none() {
                return Err(MusketError::Cli {
                    message: format!("The url \"{url}\" cannot be sent to a non-existing destination. Set, at least, one valid destination."),
                });
            }

            let cfg = config::configure()?;
            let tags = tags.unwrap_or_default();
            let destinations = destination.unwrap_or_default();

            for target in destinations {
                match target {
                    Destinations::All => {
                        success_messages.push(
                            bluesky_shooter(&cfg, &url, tags.clone(), commentary.as_ref()).await?,
                        );
                        success_messages.push(
                            linkedin_shooter(&cfg, &url, tags.clone(), commentary.as_ref()).await?,
                        );
                        success_messages.push(
                            mastodon_shooter(&cfg, &url, tags.clone(), commentary.as_ref()).await?,
                        );
                        success_messages.push(turso_shooter(&cfg, &url, tags.clone(), None).await?);
                    }
                    Destinations::Bluesky => {
                        success_messages.push(
                            bluesky_shooter(&cfg, &url, tags.clone(), commentary.as_ref()).await?,
                        );
                    }
                    Destinations::LinkedIn => {
                        success_messages.push(
                            linkedin_shooter(&cfg, &url, tags.clone(), commentary.as_ref()).await?,
                        );
                    }
                    Destinations::Mastodon => {
                        success_messages.push(
                            mastodon_shooter(&cfg, &url, tags.clone(), commentary.as_ref()).await?,
                        );
                    }
                    Destinations::Turso => {
                        success_messages.push(turso_shooter(&cfg, &url, tags.clone(), None).await?);
                    }
                }
            }
        }
    }
    Ok(success_messages)
}
