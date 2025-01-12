use crate::config;

pub mod bluesky;
pub mod linkedin;
pub mod mastodon;
pub mod turso;

pub trait Shooter {
    async fn shoot(
        &self,
        cfg: &config::Configuration,
        url: &str,
        vector_of_tags: &[String],
        commentary: Option<&String>,
    ) -> Result<String, crate::errors::MusketError>;
}
