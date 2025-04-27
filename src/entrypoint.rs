use crate::cli::{Cli, Command};
use crate::config;
use crate::destinations::Destinations;
use crate::errors::MusketError;
use crate::shooters::{bluesky_shooter, linkedin_shooter, mastodon_shooter, turso_shooter};
use crate::sources::{instapaper, Bookmark};
use clap::Parser;
use tracing::{debug, level_filters::LevelFilter};
use tracing_subscriber::EnvFilter;

/// Runs the main logic of the application.
///
/// # Errors
///
/// This function will return an error if any of the commands fail.
#[allow(clippy::too_many_lines)]
pub async fn run() -> Result<Vec<String>, MusketError> {
    let mut success_messages: Vec<String> = vec![];
    let cli = Cli::parse();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
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
            from,
            destination,
            tags,
            commentary,
            language,
        } => {
            debug!("Inside run function. Command::Fire");

            if !config::configuration_exists()? {
                return Err(MusketError::Cli {
                    message: "The configuration file does not exist. To send any URL to any destination, please first run the musket init command and next fill the configuration file.".to_string(),
                });
            }

            if url.is_none() && from.is_none() {
                return Err(MusketError::Cli {
                    message: "Neither the url nor the from flags are present. Set, at least, one of them.".to_string(),
                });
            }

            if destination.is_none() {
                return Err(MusketError::Cli {
                    message: "The url cannot be sent to a non-existing destination. Set, at least, one valid destination.".to_string(),
                });
            }

            let cfg = config::configure()?;
            let mut tags = tags.unwrap_or_default();
            let destinations = destination.unwrap_or_default();
            let mut url = url.unwrap_or_default();
            let mut bookmark: Bookmark = Bookmark {
                id: 0,
                url: String::new(),
                tags: vec![],
            };

            if from.is_some() {
                let instapaper = instapaper::Instapaper::new(
                    &cfg.instapaper.username,
                    &cfg.instapaper.password,
                    &cfg.instapaper.consumer_key,
                    &cfg.instapaper.consumer_secret,
                );
                bookmark = instapaper.get_bookmark().await?;
                url = bookmark.url;
                tags = bookmark.tags;
                success_messages.push(format!(
                    "The bookmark \"{0}\" with this url \"{1}\" has been obtained from Instapaper.",
                    bookmark.id, url
                ));
            }

            for target in destinations {
                match target {
                    Destinations::All => {
                        success_messages.push(
                            bluesky_shooter(
                                &cfg,
                                &url,
                                tags.clone(),
                                commentary.as_ref(),
                                language.as_ref(),
                            )
                            .await?,
                        );
                        success_messages.push(
                            linkedin_shooter(
                                &cfg,
                                &url,
                                tags.clone(),
                                commentary.as_ref(),
                                language.as_ref(),
                            )
                            .await?,
                        );
                        success_messages.push(
                            mastodon_shooter(
                                &cfg,
                                &url,
                                tags.clone(),
                                commentary.as_ref(),
                                language.as_ref(),
                            )
                            .await?,
                        );
                        success_messages.push(turso_shooter(&cfg, &url, tags.clone(), None).await?);
                    }
                    Destinations::Bluesky => {
                        success_messages.push(
                            bluesky_shooter(
                                &cfg,
                                &url,
                                tags.clone(),
                                commentary.as_ref(),
                                language.as_ref(),
                            )
                            .await?,
                        );
                    }
                    Destinations::LinkedIn => {
                        success_messages.push(
                            linkedin_shooter(
                                &cfg,
                                &url,
                                tags.clone(),
                                commentary.as_ref(),
                                language.as_ref(),
                            )
                            .await?,
                        );
                    }
                    Destinations::Mastodon => {
                        success_messages.push(
                            mastodon_shooter(
                                &cfg,
                                &url,
                                tags.clone(),
                                commentary.as_ref(),
                                language.as_ref(),
                            )
                            .await?,
                        );
                    }
                    Destinations::Turso => {
                        success_messages.push(turso_shooter(&cfg, &url, tags.clone(), None).await?);
                    }
                }
            }

            if from.is_some() {
                let instapaper = instapaper::Instapaper::new(
                    &cfg.instapaper.username,
                    &cfg.instapaper.password,
                    &cfg.instapaper.consumer_key,
                    &cfg.instapaper.consumer_secret,
                );
                instapaper.delete_bookmark(bookmark.id).await?;
                success_messages.push(format!(
                    "The bookmark \"{0}\" with this url \"{1}\" has been deleted of Instapaper.",
                    bookmark.id, url
                ));
            }
        }
    }
    Ok(success_messages)
}
