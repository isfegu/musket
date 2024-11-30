use crate::{
    config,
    destinations::{turso::Turso, Destination},
};

pub async fn execute(
    cfg: &config::Configuration,
    url: &str,
    vector_of_tags: &[String],
) -> Result<(), String> {
    let turso = Turso {
        url: cfg.turso.url.to_string(),
        token: cfg.turso.token.to_string(),
    };
    turso.fire(url, vector_of_tags).await?;
    println!("The url \"{}\" has been sent to Turso.", url);
    Ok(())
}
