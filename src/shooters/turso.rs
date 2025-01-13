use super::Shooter;
use crate::{
    config,
    destinations::{turso::Turso, Destination},
    errors::MusketError,
};

pub struct TursoShooter;

impl Shooter for TursoShooter {
    async fn shoot(
        &self,
        cfg: &config::Configuration,
        url: &str,
        tags: Vec<String>,
        _commentary: Option<&String>,
    ) -> Result<String, MusketError> {
        if !cfg.turso.enabled {
            return Ok(format!(
                "The url \"{url}\" cannot be sent to Turso because this destination is disabled."
            ));
        }

        let turso = Turso {
            database: cfg.turso.database.to_string(),
            token: cfg.turso.token.to_string(),
            url: url.to_string(),
            tags,
        };
        turso.fire().await?;

        Ok(format!("The url \"{url}\" has been sent to Turso."))
    }
}
