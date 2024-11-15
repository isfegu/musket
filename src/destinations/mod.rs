pub trait Destination {
    async fn fire(&self, url: &str, tags: &[String]) -> Result<(), Box<dyn std::error::Error>>;
}

pub mod turso;
