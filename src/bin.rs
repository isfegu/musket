use clap::{Parser, Subcommand};
use destinations::Destination;

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

        /// Place to send (publish, save, etc.) the URL
        #[arg(short, long)]
        destination: Vec<DestinationsLabels>,

        /// Tags separated by comas
        #[arg(short, long)]
        tags: Option<String>,
    },
    /// TO DO: write something about FireAtWill Command
    ///
    /// TO DO: write a description
    FireAtWill {
        /// URL to send to the destinations
        #[arg(short, long)]
        url: String,

        /// Tags separated by comas
        #[arg(short, long)]
        tags: Option<String>,
    },
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum DestinationsLabels {
    All,
    X,
    Linkedin,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", &args.cmd);

    let linked_in = destinations::LinkedIn;
    linked_in.shot();

    let x = destinations::X;
    x.shot();
}
