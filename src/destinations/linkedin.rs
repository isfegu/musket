use super::{Destination, DestinationError};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::debug;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct LinkedinConfiguration {
    pub token: String,
    pub author: String,
    pub commentary: String,
    pub language: String,
    pub visibility: String,
    pub enabled: bool,
}
pub struct LinkedIn {
    pub configuration: LinkedinConfiguration,
    pub url: String,
    pub tags: Vec<String>,
    pub commentary: String,
    pub language: String,
}

impl From<reqwest::Error> for DestinationError {
    fn from(e: reqwest::Error) -> Self {
        DestinationError::LinkedIn {
            message: format!("The url cannot be sent to LinkedIn due to {e}."),
        }
    }
}

#[derive(Deserialize)]
struct LinkedInResponse {
    message: String,
}

impl Destination for LinkedIn {
    async fn fire(&self) -> Result<(), DestinationError> {
        debug!("Inside fire function. LinkedIn destination");

        let mut share_commentary = self.commentary.clone();

        if !self.tags.is_empty() {
            let tags_joined = self.tags.join(", #");
            share_commentary = format!("{}\n\n#{}", self.commentary.clone(), tags_joined);
        }

        let json = json!({
            "author": self.configuration.author.clone(),
            "lifecycleState": "PUBLISHED",
            "specificContent": {
                "com.linkedin.ugc.ShareContent": {
                    "shareCommentary": {
                        "inferredLocale": self.language,
                        "text": share_commentary
                    },
                    "shareMediaCategory": "ARTICLE",
                    "media": [
                        {
                            "status": "READY",
                            "originalUrl": self.url,
                        }
                    ]
                }
            },
            "visibility": {
                "com.linkedin.ugc.MemberNetworkVisibility": self.configuration.visibility.clone()
            }
        });

        let client = reqwest::Client::new();
        let response: reqwest::Response = client
            .post("https://api.linkedin.com/v2/ugcPosts")
            .header(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", self.configuration.token.clone()),
            )
            .header("X-Restli-Protocol-Version", "2.0.0")
            .json(&json)
            .send()
            .await?;

        if response.status().is_server_error() || response.status().is_client_error() {
            let response_content = response.json::<LinkedInResponse>().await?;
            return Err(DestinationError::LinkedIn {
                message: format!(
                    "The url cannot be sent to LinkedIn due to {}.",
                    response_content.message
                ),
            });
        }

        Ok(())
    }
}
