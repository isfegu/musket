use super::{Destination, DestinationError};
use serde_json::json;

pub struct LinkedIn {
    pub token: String,
    pub author: String,
    pub url: String,
    pub tags: Vec<String>,
    pub commentary: String,
    pub visibility: String,
}

impl From<reqwest::Error> for DestinationError {
    fn from(e: reqwest::Error) -> Self {
        DestinationError::LinkedIn {
            message: format!("The url cannot be sent to LinkedIn due to {e}."),
        }
    }
}

impl Destination for LinkedIn {
    async fn fire(&self) -> Result<(), DestinationError> {
        let mut share_commentary = self.commentary.clone();

        if !self.tags.is_empty() {
            let tags_joined = self.tags.join(", #");
            share_commentary = format!("{}\n\n#{}", self.commentary.clone(), tags_joined);
        }

        let json = json!({
            "author": self.author.clone(),
            "lifecycleState": "PUBLISHED",
            "specificContent": {
                "com.linkedin.ugc.ShareContent": {
                    "shareCommentary": {
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
                "com.linkedin.ugc.MemberNetworkVisibility": self.visibility.clone()
            }
        });

        let client = reqwest::Client::new();
        let _response: reqwest::Response = client
            .post("https://api.linkedin.com/v2/ugcPosts")
            .header(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", self.token.clone()),
            )
            .header("X-Restli-Protocol-Version", "2.0.0")
            .json(&json)
            .send()
            .await?;

        Ok(())
    }
}
