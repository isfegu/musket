use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Use the Fire command to send a URL to one or more destinations. A list of tags can be specified.
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
    /// Use the FireAtWill command to send a URL to all the destinations enabled by Musket. A list of tags can be specified.
    FireAtWill {
        /// URL to send to the destinations
        #[arg(short, long)]
        url: String,

        /// The tags to be used in the destinations.
        #[arg(short, long, value_delimiter = ',')]
        tags: Option<Vec<String>>,
    },
}

/// Enum with all the enabled destinations
#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Destinations {
    All,
    Turso,
}
