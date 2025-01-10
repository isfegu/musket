#[derive(Debug)]
pub enum DestinationError {
    Bluesky { message: String },
    LinkedIn { message: String },
    Turso { message: String },
}

impl std::fmt::Display for DestinationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use DestinationError::{Bluesky, LinkedIn, Turso};
        let output = match self {
            Bluesky { message: m } | LinkedIn { message: m } | Turso { message: m } => m,
        };
        write!(f, "{output}")
    }
}
