use clap::Parser;
use cli::{Cli, Command, Destinations};

mod cli;
mod commands;
mod config;
mod destinations;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.cmd {
        Command::Load => match config::configure() {
            Ok(_) => {
                println!(
                  "The configuration file has been created here: \"{}\". To start using Musket, please fill in the configuration file with your data.",
                  config::get_configuration_path()
                      .unwrap_or_default()
                      .to_string_lossy()
              );
            }
            Err(e) => {
                eprintln!("The configuration file cannot be created due to \"{}\".", e);
                std::process::exit(1);
            }
        },
        Command::Fire {
            url,
            destination,
            tags,
        } => {
            if destination.is_none() {
                eprintln!("The url \"{}\" cannot be sent to a non-existing destination. Set, at least, one valid destination.", url);
                std::process::exit(1);
            }

            let cfg = config::configure()?;

            let vector_of_tags = tags.unwrap_or_default();
            let destinations = destination.unwrap_or_default();

            for target in destinations {
                match target {
                    Destinations::All => {
                        commands::turso::execute(&cfg, &url, &vector_of_tags).await?;
                        commands::linkedin::execute(&cfg, &url, &vector_of_tags).await?;
                    }
                    Destinations::Turso => {
                        commands::turso::execute(&cfg, &url, &vector_of_tags).await?;
                    }
                    Destinations::LinkedIn => {
                        commands::linkedin::execute(&cfg, &url, &vector_of_tags).await?;
                    }
                }
            }
        }
    }
    Ok(())
}
