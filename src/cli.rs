use clap::{Parser, Subcommand};

use crate::destinations::Destinations;
use crate::sources::Sources;

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
    /// Use the Fire command to send a URL to one or more destinations.
    Fire {
        /// The URL to send to the destinations. The url flag is mandatory if the source flag is not present.
        #[arg(short, long)]
        url: Option<String>,

        /// The source from which to obtain the links that will be sent to the destinations. The source flag is mandatory if the url flag is not present.
        #[arg(short, long)]
        from: Option<Sources>,

        /// The place to send (publish, save, etc.) the URL. At least, one destination must be choosed.
        #[arg(short, long, value_delimiter = ',')]
        destination: Option<Vec<Destinations>>,

        /// The tags to be used in the destinations. The tags are optional.
        #[arg(short, long, value_delimiter = ',')]
        tags: Option<Vec<String>>,

        /// The comment to publish along with the URL (if destination allows it). The comment is optional.
        #[arg(short, long)]
        commentary: Option<String>,

        /// The language to use in the destination. The language is optional.
        #[arg(short, long)]
        language: Option<String>,
    },
}
