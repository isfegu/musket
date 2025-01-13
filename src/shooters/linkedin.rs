use super::Shooter;
use crate::{
    config,
    destinations::{linkedin::LinkedIn, Destination},
    errors::MusketError,
};

pub struct LinkedInShooter;

impl Shooter for LinkedInShooter {
    async fn shoot(
        &self,
        cfg: &config::Configuration,
        url: &str,
        vector_of_tags: &[String],
        commentary: Option<&String>,
    ) -> Result<String, MusketError> {
        if !cfg.linkedin.enabled {
            return Ok(format!(
                "The url \"{url}\" cannot be sent to LinkedIn because this destination is disabled."
            ));
        }

        let linkedin = LinkedIn {
            token: cfg.linkedin.token.to_string(),
            author: cfg.linkedin.author.to_string(),
            commentary: commentary.unwrap_or(&cfg.linkedin.commentary).to_string(),
            visibility: cfg.linkedin.visibility.to_string(),
        };
        linkedin.fire(url, vector_of_tags).await?;
        Ok(format!("The url \"{url}\" has been sent to LinkedIn."))
    }
}
