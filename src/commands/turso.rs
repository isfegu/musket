use crate::{
    config,
    destinations::{turso::Turso, Destination},
};

pub async fn execute(
    cfg: &config::Configuration,
    url: &str,
    vector_of_tags: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut turso = Turso::new();
    turso.configure(&cfg.turso.url, &cfg.turso.token);
    turso.fire(url, vector_of_tags).await?;
    Ok(())
}
