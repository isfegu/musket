use clap::Parser;
use cli::{Cli, Command, Destinations};

mod cli;
mod commands;
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
            if destination.is_none() {
                eprintln!("The url \"{}\" cannot be sent to a non-existing destination. Set, at least, one valid destination.", url);
                std::process::exit(1);
            }

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
        Command::FireAtWill { url, tags } => {
            let vector_of_tags = tags.unwrap_or_default();

            commands::turso::execute(&cfg, &url, &vector_of_tags).await?;
            commands::linkedin::execute(&cfg, &url, &vector_of_tags).await?;
        }
    }
    Ok(())
}

/*
https://github.com/kohbis/rslack/blob/main/src/slack/mod.rs
https://learn.microsoft.com/es-es/linkedin/consumer/integrations/self-serve/share-on-linkedin
https://learn.microsoft.com/es-es/linkedin/consumer/integrations/self-serve/sign-in-with-linkedin-v2
https://github.com/linkedin-developers/linkedin-api-js-client
https://docs.rs/reqwest/0.12.9/reqwest/
https://blog.logrocket.com/making-http-requests-rust-reqwest/
https://github.com/seanmonstar/reqwest/blob/master/examples/form.rs
*/
