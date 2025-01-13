use super::Shooter;
use crate::{
    config,
    destinations::{mastodon::Mastodon, Destination},
    errors::MusketError,
};

pub struct MastodonShooter;

impl Shooter for MastodonShooter {
    async fn shoot(
        &self,
        cfg: &config::Configuration,
        url: &str,
        tags: Vec<String>,
        commentary: Option<&String>,
    ) -> Result<String, MusketError> {
        if !cfg.mastodon.enabled {
            return Ok(format!(
                "The url \"{url}\" cannot be sent to Mastodon because this destination is disabled."
            ));
        }

        let mastodon = Mastodon {
            server: cfg.mastodon.server.to_string(),
            token: cfg.mastodon.token.to_string(),
            url: url.to_string(),
            tags,
            commentary: commentary.unwrap_or(&cfg.mastodon.commentary).to_string(),
        };

        mastodon.fire().await?;

        Ok(format!("The url \"{url}\" has been sent to Mastodon."))
    }
}
