use crate::{
    config,
    destinations::{turso::Turso, Destination},
    errors::*,
};

pub async fn execute(
    cfg: &config::Configuration,
    url: &str,
    vector_of_tags: &[String],
) -> Result<String, MusketError> {
    let turso = Turso {
        url: cfg.turso.url.to_string(),
        token: cfg.turso.token.to_string(),
    };
    turso.fire(url, vector_of_tags).await?;
    Ok(format!("The url \"{}\" has been sent to Turso.", url))
}
