use crate::{config::ConfigurationError, destinations::errors::DestinationError};

#[derive(Debug)]
pub enum MusketError {
    Destination { message: String },
    Configuration { message: String },
    Cli { message: String },
}

impl std::error::Error for MusketError {}

impl std::fmt::Display for MusketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use MusketError::*;
        let output = match self {
            Destination { message: m } => m,
            Configuration { message: m } => m,
            Cli { message: m } => m,
        };
        write!(f, "{output}")
    }
}

impl From<ConfigurationError> for MusketError {
    fn from(e: ConfigurationError) -> Self {
        MusketError::Configuration {
            message: format!("{}.", e),
        }
    }
}

impl From<DestinationError> for MusketError {
    fn from(e: DestinationError) -> Self {
        MusketError::Destination {
            message: format!("{}.", e),
        }
    }
}
