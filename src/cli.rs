use clap::{Parser, Subcommand};

use crate::destinations::Destinations;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Use the Init command to create the configuration file in the user's home.
    Init {
        /// Force the creation of the configuration file. If the configuration file already exists, it will be overwritten.
        #[arg(
            short,
            long,
            default_missing_value("true"),
            default_value("false"),
            num_args(0)
        )]
        force: Option<bool>,
    },
    /// Use the Fire command to send a URL to one or more destinations. A list of tags can be specified.
    Fire {
        /// The URL to send to the destinations. The URL is mandatory.
        #[arg(short, long)]
        url: String,

        /// The place to send (publish, save, etc.) the URL. At least, one destination must be choosed.
        #[arg(short, long, value_delimiter = ',')]
        destination: Option<Vec<Destinations>>,

        /// The tags to be used in the destinations. The tags are optional.
        #[arg(short, long, value_delimiter = ',')]
        tags: Option<Vec<String>>,

        /// The comment to publish along with the URL (if destination allows it). The comment is optional.
        #[arg(short, long)]
        commentary: Option<String>,
    },
}
