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
) -> Result<String, MusketError> {
    if !cfg.bluesky.enabled {
        return Ok(format!(
            "The url \"{url}\" cannot be sent to Bluesky because this destination is disabled."
        ));
    }

    let bluesky = Bluesky {
        configuration: cfg.bluesky.clone(),
        url: url.to_string(),
        tags,
        commentary: commentary.unwrap_or(&cfg.bluesky.commentary).to_string(),
    };

    bluesky.fire().await?;

    Ok(format!("The url \"{url}\" has been sent to Bluesky."))
}

pub async fn linkedin_shooter(
    cfg: &config::Configuration,
    url: &str,
    tags: Vec<String>,
    commentary: Option<&String>,
) -> Result<String, MusketError> {
    if !cfg.linkedin.enabled {
        return Ok(format!(
            "The url \"{url}\" cannot be sent to LinkedIn because this destination is disabled."
        ));
    }

    let linkedin = LinkedIn {
        configuration: cfg.linkedin.clone(),
        url: url.to_string(),
        tags,
        commentary: commentary.unwrap_or(&cfg.linkedin.commentary).to_string(),
    };

    linkedin.fire().await?;

    Ok(format!("The url \"{url}\" has been sent to LinkedIn."))
}

pub async fn mastodon_shooter(
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
        configuration: cfg.mastodon.clone(),
        url: url.to_string(),
        tags,
        commentary: commentary.unwrap_or(&cfg.mastodon.commentary).to_string(),
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
    if !cfg.turso.enabled {
        return Ok(format!(
            "The url \"{url}\" cannot be sent to Turso because this destination is disabled."
        ));
    }

    let turso = Turso {
        configuration: cfg.turso.clone(),
        url: url.to_string(),
        tags,
    };

    turso.fire().await?;

    Ok(format!("The url \"{url}\" has been sent to Turso."))
}
