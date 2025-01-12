use crate::{
    config,
    destinations::{mastodon::Mastodon, Destination},
    errors::MusketError,
};

pub async fn execute(
    cfg: &config::Configuration,
    url: &str,
    vector_of_tags: &[String],
    commentary: Option<&String>,
) -> Result<String, MusketError> {
    let mastodon = Mastodon {
        server: cfg.mastodon.server.to_string(),
        token: cfg.mastodon.token.to_string(),
        commentary: commentary.unwrap_or(&cfg.mastodon.commentary).to_string(),
    };
    mastodon.fire(url, vector_of_tags).await?;
    Ok(format!("The url \"{url}\" has been sent to Mastodon."))
}
