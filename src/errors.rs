use crate::{
    config::ConfigurationError, destinations::errors::DestinationError,
    sources::errors::SourceError,
};

#[derive(Debug)]
pub enum MusketError {
    Cli { message: String },
    Configuration { message: String },
    Destination { message: String },
    Source { message: String },
}

impl std::error::Error for MusketError {}

impl std::fmt::Display for MusketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use MusketError::{Cli, Configuration, Destination, Source};
        let output = match self {
            Cli { message: m }
            | Configuration { message: m }
            | Destination { message: m }
            | Source { message: m } => m,
        };
        write!(f, "{output}")
    }
}

impl From<ConfigurationError> for MusketError {
    fn from(e: ConfigurationError) -> Self {
        MusketError::Configuration {
            message: format!("{e}."),
        }
    }
}

impl From<DestinationError> for MusketError {
    fn from(e: DestinationError) -> Self {
        MusketError::Destination {
            message: format!("{e}."),
        }
    }
}

impl From<SourceError> for MusketError {
    fn from(e: SourceError) -> Self {
        MusketError::Source {
            message: format!("{e}."),
        }
    }
}
