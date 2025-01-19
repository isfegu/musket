use tracing::debug;

use crate::{
    config,
    destinations::{
        bluesky::Bluesky, linkedin::LinkedIn, mastodon::Mastodon, turso::Turso, Destination,
    },
    errors::MusketError,
};

pub async fn bluesky_shooter(
    cfg: &config::Configuration,
    url: &str,
    tags: Vec<String>,
    commentary: Option<&String>,
    language: Option<&String>,
) -> Result<String, MusketError> {
    debug!("Inside bluesky_shooter function");

    if !cfg.bluesky.enabled {
        return Err(MusketError::Destination {
            message: format!(
                "The url \"{url}\" cannot be sent to Bluesky because this destination is disabled."
            ),
        });
    }

    let bluesky = Bluesky {
        configuration: cfg.bluesky.clone(),
        url: url.to_string(),
        tags,
        commentary: commentary.unwrap_or(&cfg.bluesky.commentary).to_string(),
        language: language.unwrap_or(&cfg.bluesky.language).to_string(),
    };

    bluesky.fire().await?;

    Ok(format!("The url \"{url}\" has been sent to Bluesky."))
}

pub async fn linkedin_shooter(
    cfg: &config::Configuration,
    url: &str,
    tags: Vec<String>,
    commentary: Option<&String>,
    language: Option<&String>,
) -> Result<String, MusketError> {
    debug!("Inside linkedin_shooter function");

    if !cfg.linkedin.enabled {
        return Err(MusketError::Destination {
            message: format!(
                "The url \"{url}\" cannot be sent to LinkedIn because this destination is disabled."
            ),
        });
    }

    let linkedin = LinkedIn {
        configuration: cfg.linkedin.clone(),
        url: url.to_string(),
        tags,
        commentary: commentary.unwrap_or(&cfg.linkedin.commentary).to_string(),
        language: language.unwrap_or(&cfg.linkedin.language).to_string(),
    };

    linkedin.fire().await?;

    Ok(format!("The url \"{url}\" has been sent to LinkedIn."))
}

pub async fn mastodon_shooter(
    cfg: &config::Configuration,
    url: &str,
    tags: Vec<String>,
    commentary: Option<&String>,
    language: Option<&String>,
) -> Result<String, MusketError> {
    debug!("Inside mastodon_shooter function");

    if !cfg.mastodon.enabled {
        return Err(MusketError::Destination {
            message: format!(
                "The url \"{url}\" cannot be sent to Mastodon because this destination is disabled."
            ),
        });
    }

    let mastodon = Mastodon {
        configuration: cfg.mastodon.clone(),
        url: url.to_string(),
        tags,
        commentary: commentary.unwrap_or(&cfg.mastodon.commentary).to_string(),
        language: language.unwrap_or(&cfg.mastodon.language).to_string(),
    };

    mastodon.fire().await?;

    Ok(format!("The url \"{url}\" has been sent to Mastodon."))
}

pub async fn turso_shooter(
    cfg: &config::Configuration,
    url: &str,
    tags: Vec<String>,
    _commentary: Option<&String>,
) -> Result<String, MusketError> {
    debug!("Inside turso_shooter function");

    if !cfg.turso.enabled {
        return Err(MusketError::Destination {
            message: format!(
                "The url \"{url}\" cannot be sent to Turso because this destination is disabled."
            ),
        });
    }

    let turso = Turso {
        configuration: cfg.turso.clone(),
        url: url.to_string(),
        tags,
    };

    turso.fire().await?;

    Ok(format!("The url \"{url}\" has been sent to Turso."))
}
