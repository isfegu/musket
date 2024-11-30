use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub linkedin: LinkedinConfiguration,
    pub turso: TursoConfiguration,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct TursoConfiguration {
    pub url: String,
    pub token: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct LinkedinConfiguration {
    pub token: String,
    pub author: String,
    pub share_commentary: String,
    pub visibility: String,
}

pub fn configure() -> Result<Configuration, confy::ConfyError> {
    let cfg: Configuration = confy::load("musket", "config")?;
    Ok(cfg)
}

pub fn get_configuration_path() -> Result<PathBuf, confy::ConfyError> {
    confy::get_configuration_file_path("musket", "config")
}
