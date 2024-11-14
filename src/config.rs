use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub turso: TursoConfiguration,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct TursoConfiguration {
    pub url: String,
    pub token: String,
}

pub fn configure() -> Result<Configuration, confy::ConfyError> {
    let cfg: Configuration = confy::load("musket", "config")?;
    Ok(cfg)
}
