use crate::{
    config,
    destinations::{linkedin::LinkedIn, Destination},
};

pub async fn execute(
    cfg: &config::Configuration,
    url: &str,
    vector_of_tags: &[String],
) -> Result<(), String> {
    let linkedin = LinkedIn {
        token: cfg.linkedin.token.to_string(),
        author: cfg.linkedin.author.to_string(),
        share_commentary: cfg.linkedin.share_commentary.to_string(),
        visibility: cfg.linkedin.visibility.to_string(),
    };
    linkedin.fire(url, vector_of_tags).await?;
    println!("The url \"{}\" has been sent to LinkedIn.", url);
    Ok(())
}
