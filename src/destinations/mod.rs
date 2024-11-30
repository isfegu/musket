pub trait Destination {
    async fn fire(&self, url: &str, tags: &[String]) -> Result<(), String>;
}

pub mod linkedin;
pub mod turso;
