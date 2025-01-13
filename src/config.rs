use confy::ConfyError;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct BlueskyConfiguration {
    pub identifier: String,
    pub password: String,
    pub commentary: String,
    pub enabled: bool,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct LinkedinConfiguration {
    pub token: String,
    pub author: String,
    pub commentary: String,
    pub visibility: String,
    pub enabled: bool,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct MastodonConfiguration {
    pub server: String,
    pub token: String,
    pub commentary: String,
    pub enabled: bool,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct TursoConfiguration {
    pub url: String,
    pub token: String,
    pub enabled: bool,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub bluesky: BlueskyConfiguration,
    pub linkedin: LinkedinConfiguration,
    pub mastodon: MastodonConfiguration,
    pub turso: TursoConfiguration,
}

pub fn configuration_exists() -> Result<bool, ConfigurationError> {
    let configuration_path = confy::get_configuration_file_path("musket", "config")?;
    Ok(configuration_path.exists())
}

pub fn configure() -> Result<Configuration, ConfigurationError> {
    Ok(confy::load("musket", "config")?)
}

pub fn get_configuration_path() -> Result<PathBuf, ConfigurationError> {
    Ok(confy::get_configuration_file_path("musket", "config")?)
}

#[derive(Debug)]
pub enum ConfigurationError {
    Configuration { message: String },
}

impl std::fmt::Display for ConfigurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ConfigurationError::Configuration;
        let output = match self {
            Configuration { message: m } => m,
        };
        write!(f, "{output}")
    }
}

impl From<ConfyError> for ConfigurationError {
    fn from(e: ConfyError) -> Self {
        ConfigurationError::Configuration {
            message: format!("The configuration file cannot be used due to \"{e}\"."),
        }
    }
}
