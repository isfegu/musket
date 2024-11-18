use crate::{
    config,
    destinations::{linkedin::LinkedIn, Destination},
};

pub async fn execute(
    cfg: &config::Configuration,
    url: &str,
    vector_of_tags: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut linkedin = LinkedIn::new();
    linkedin.configure(
        &cfg.linkedin.token,
        &cfg.linkedin.author,
        &cfg.linkedin.share_commentary,
        &cfg.linkedin.visibility,
    );
    linkedin.fire(url, vector_of_tags).await?;
    println!("The url \"{}\" has been sent to LinkedIn.", url);
    Ok(())
}
