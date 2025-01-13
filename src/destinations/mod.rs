pub mod bluesky;
pub mod errors;
pub mod linkedin;
pub mod mastodon;
pub mod turso;

use errors::DestinationError;

pub trait Destination {
    async fn fire(&self) -> Result<(), DestinationError>;
}
