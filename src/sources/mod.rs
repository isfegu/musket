pub mod errors;
pub mod instapaper;

use errors::SourceError;

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Sources {
    Instapaper,
}

#[derive(Debug)]
pub struct Bookmark {
    pub id: i64,
    pub url: String,
    pub tags: Vec<String>,
}
