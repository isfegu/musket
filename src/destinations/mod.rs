pub mod errors;
pub mod linkedin;
pub mod turso;

use errors::DestinationError;

pub trait Destination {
    async fn fire(&self, url: &str, tags: &[String]) -> Result<(), DestinationError>;
}
