use clap::Parser;
use cli::{Cli, Command, Destinations};
use destinations::{turso::Turso, Destination};

mod cli;
mod config;
mod destinations;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg: config::Configuration = config::configure()?;

    let cli = Cli::parse();

    match cli.cmd {
        Command::Fire {
            url,
            destination,
            tags,
        } => {
            let vector_of_tags = tags.unwrap_or_default();
            match destination {
                Some(destinations) => {
                    for target in destinations {
                        match target {
                            Destinations::All => {}
                            Destinations::Turso => {
                                let mut turso = Turso::new();
                                turso.configure(&cfg.turso.url, &cfg.turso.token);
                                turso.fire(&url, &vector_of_tags).await?;
                            }
                        }
                    }
                }
                None => {
                    eprintln!(
                        "The url \"{}\" cannot be sent to a non-existing destination. Set, at least, one valid destination.",
                        url
                    );
                    std::process::exit(1);
                }
            }
        }
        Command::FireAtWill { url: _, tags: _ } => {
            todo!();
        }
    }
    Ok(())
}
