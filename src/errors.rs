#[derive(Debug)]
pub enum Musket {
    Destination { message: String },
    Configuration { message: String },
    Cli { message: String },
}

impl std::error::Error for Musket {}

impl std::fmt::Display for Musket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Musket::*;
        let output = match self {
            Destination { message: m } => m,
            Configuration { message: m } => m,
            Cli { message: m } => m,
        };
        write!(f, "{output}")
    }
}
