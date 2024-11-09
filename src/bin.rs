use clap::{Parser, Subcommand};

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
    Foo,
    Bar,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args.cmd);

    match args.cmd {
        Commands::Fire {
            url,
            destination,
            tags,
        } => {
            let vector_of_tags = tags.unwrap_or_default();

            match destination {
                Some(destinations) => {
                    destinations
                        .iter()
                        .for_each(|target| fire_a_bullet(&url, target, &vector_of_tags));
                }
                None => {
                    println!(
                        "The url '{}' cannot be sent to a non-existing destination. Set at least one destination.",
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

fn fire_a_bullet(url: &str, target: &Destinations, tags: &Vec<String>) {
    match target {
        Destinations::All => {
            println!(
                "Send this url {:?} to this destination {:?} using this tags {:?}",
                url,
                Destinations::All,
                tags
            );
        }
        Destinations::Foo => {
            println!(
                "Send this url {:?} to this destination {:?} using this tags {:?}",
                url,
                Destinations::Foo,
                tags
            );
        }
        Destinations::Bar => {
            println!(
                "Send this url {:?} to this destination {:?} using this tags {:?}",
                url,
                Destinations::Bar,
                tags
            );
        }
    }
}
