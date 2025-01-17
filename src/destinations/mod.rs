pub mod bluesky;
pub mod errors;
pub mod linkedin;
pub mod mastodon;
pub mod turso;

use errors::DestinationError;

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Destinations {
    All,
    Bluesky,
    LinkedIn,
    Mastodon,
    Turso,
}

impl std::fmt::Display for Destinations {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

pub trait Destination {
    async fn fire(&self) -> Result<(), DestinationError>;
}
