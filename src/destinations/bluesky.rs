use super::{Destination, DestinationError};
use bsky_sdk::{
    api::{
        types::string::{Datetime, Language},
        xrpc,
    },
    rich_text::RichText,
    BskyAgent,
};
use serde::{Deserialize, Serialize};
use tracing::debug;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BlueskyConfiguration {
    pub identifier: String,
    pub password: String,
    pub commentary: String,
    pub language: String,
    pub enabled: bool,
}
pub struct Bluesky {
    pub configuration: BlueskyConfiguration,
    pub url: String,
    pub tags: Vec<String>,
    pub commentary: String,
    pub language: String,
}

impl From<bsky_sdk::Error> for DestinationError {
    fn from(e: bsky_sdk::Error) -> Self {
        DestinationError::LinkedIn {
            message: format!("The url cannot be sent to Bluesky due to {e}."),
        }
    }
}

impl From<xrpc::Error<bsky_sdk::api::com::atproto::server::create_session::Error>>
    for DestinationError
{
    fn from(e: xrpc::Error<bsky_sdk::api::com::atproto::server::create_session::Error>) -> Self {
        DestinationError::LinkedIn {
            message: format!("The url cannot be sent to Bluesky due to {e}."),
        }
    }
}

impl Destination for Bluesky {
    async fn fire(&self) -> Result<(), DestinationError> {
        debug!("Inside fire function. Bluesky destination");

        let agent = BskyAgent::builder().build().await?;
        agent
            .login(
                self.configuration.identifier.as_str(),
                self.configuration.password.as_str(),
            )
            .await?;

        let mut rich_text_content = format!("{} {}", self.commentary, self.url);

        if !self.tags.is_empty() {
            let tags_joined = self.tags.join(" #");
            rich_text_content = format!("{}\n#{}", rich_text_content, &tags_joined);
        }

        let rt = RichText::new_with_detect_facets(rich_text_content).await?;

        // @todo: Use an From implementation instead map_err to handle language conversion error
        agent
            .create_record(bsky_sdk::api::app::bsky::feed::post::RecordData {
                created_at: Datetime::now(),
                embed: None,
                entities: None,
                facets: rt.facets,
                labels: None,
                langs: Some(vec![Language::new(self.language.clone()).map_err(|e| {
                    DestinationError::Bluesky {
                        message: format!("The url cannot be sent to Bluesky due to {e}"),
                    }
                })?]),
                reply: None,
                tags: None,
                text: rt.text,
            })
            .await?;

        Ok(())
    }
}
