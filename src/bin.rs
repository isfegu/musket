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

        /// URL destination
        #[arg(short, long)]
        target: Vec<Destinations>,

        /// Tags separated by comas
        #[arg(short, long)]
        labels: Option<String>,
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
        labels: Option<String>,
    },
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum Destinations {
    All,
    X,
    Linkedin,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", &args.cmd);
    println!("Hello, world!");
}
