use crate::{
    config,
    destinations::{bluesky::Bluesky, Destination},
    errors::MusketError,
};

pub async fn execute(
    cfg: &config::Configuration,
    url: &str,
    vector_of_tags: &[String],
) -> Result<String, MusketError> {
    let bluesky = Bluesky {
        identifier: cfg.bluesky.identifier.to_string(),
        password: cfg.bluesky.password.to_string(),
        share_commentary: cfg.bluesky.share_commentary.to_string(),
    };
    bluesky.fire(url, vector_of_tags).await?;
    Ok(format!("The url \"{url}\" has been sent to Bluesky."))
}
