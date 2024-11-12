use chrono::prelude::*;
use clap::{Parser, Subcommand};
use libsql::Builder;

/// TO DO: Add description
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// TO DO: write something about Fire Command
    ///
    /// TO DO: write a description
    Fire {
        /// URL to send to the destinations
        #[arg(short, long)]
        url: String,

        /// Place to send (publish, save, etc.) the URL. At least, one destination must be choosed.
        #[arg(short, long, value_delimiter = ',')]
        destination: Option<Vec<Destinations>>,

        /// The tags to be used in the destinations.
        #[arg(short, long, value_delimiter = ',')]
        tags: Option<Vec<String>>,
    },
    /// TO DO: write something about FireAtWill Command
    ///
    /// TO DO: write a description
    FireAtWill {
        /// URL to send to the destinations
        #[arg(short, long)]
        url: String,

        /// The tags to be used in the destinations.
        #[arg(short, long, value_delimiter = ',')]
        tags: Option<Vec<String>>,
    },
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum Destinations {
    All,
    Turso,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::Fire {
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
                        fire_a_bullet(&url, &target, &vector_of_tags).await.unwrap();
                    }
                }
                None => {
                    println!(
                        "The url \"{}\" cannot be sent to a non-existing destination. Set at least one destination.",
                        url
                    );
                }
            }
        }
        _ => {
            todo!()
        }
    }
}

async fn fire_a_bullet(
    url: &str,
    target: &Destinations,
    tags: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    match target {
        Destinations::All => Ok(()),
        Destinations::Turso => {
            turso(url, tags).await?;
            Ok(())
        }
    }
}

async fn turso(url: &str, tags: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let local: DateTime<Local> = Local::now();
    let created = format!("{}", local.format("%Y-%m-%d %H:%M:%S"));

    let turso_db_url = std::env::var("TURSO_DATABASE_URL").expect("TURSO_DATABASE_URL must be set");
    let turso_db_token = std::env::var("TURSO_AUTH_TOKEN").expect("TURSO_AUTH_TOKEN must be set");

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
