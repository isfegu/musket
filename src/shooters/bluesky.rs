use super::Shooter;
use crate::{
    config,
    destinations::{bluesky::Bluesky, Destination},
    errors::MusketError,
};

pub struct BlueskyShooter;

impl Shooter for BlueskyShooter {
    async fn shoot(
        &self,
        cfg: &config::Configuration,
        url: &str,
        vector_of_tags: &[String],
        commentary: Option<&String>,
    ) -> Result<String, MusketError> {
        if !cfg.bluesky.enabled {
            return Ok(format!(
                "The url \"{url}\" cannot be sent to Bluesky because this destination is disabled."
            ));
        }

        let bluesky = Bluesky {
            identifier: cfg.bluesky.identifier.to_string(),
            password: cfg.bluesky.password.to_string(),
            commentary: commentary.unwrap_or(&cfg.bluesky.commentary).to_string(),
        };
        bluesky.fire(url, vector_of_tags).await?;
        Ok(format!("The url \"{url}\" has been sent to Bluesky."))
    }
}
