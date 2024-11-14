use chrono::prelude::*;
use clap::Parser;
use cli::{Cli, Command, Destinations};
use libsql::Builder;

mod cli;
mod config;

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
                        println!(
                            "Sending \"{}\" to {:?} using this tags \"{}\"",
                            url,
                            &target,
                            &vector_of_tags.join(", ")
                        );
                        fire_a_bullet(&cfg, &url, &target, &vector_of_tags)
                            .await
                            .unwrap();
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

async fn fire_a_bullet(
    cfg: &config::Configuration,
    url: &str,
    target: &Destinations,
    tags: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    match target {
        Destinations::All => Ok(()),
        Destinations::Turso => {
            turso(cfg, url, tags).await?;
            Ok(())
        }
    }
}

async fn turso(
    cfg: &config::Configuration,
    url: &str,
    tags: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    let local: DateTime<Local> = Local::now();
    let created = format!("{}", local.format("%Y-%m-%d %H:%M:%S"));

    let turso_db_url = cfg.turso.url.to_string();
    let turso_db_token = cfg.turso.token.to_string();

    let db = Builder::new_remote(turso_db_url, turso_db_token)
        .build()
        .await?;
    let conn = db.connect()?;
    conn.execute(
        "INSERT INTO links (url, tags, created) VALUES (:url, :tags, :created)",
        libsql::named_params! { ":url": url, ":tags": tags.join(", "), ":created": created },
    )
    .await?;

    Ok(())
}
