use super::Destination;
use serde_json::json;

#[derive(Default)]
pub struct LinkedIn {
    token: String,
    author: String,
    share_commentary: String,
    visibility: String,
}

impl LinkedIn {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn configure(
        &mut self,
        token: &str,
        author: &str,
        share_commentary: &str,
        visibility: &str,
    ) {
        self.token = token.to_string();
        self.author = author.to_string();
        self.share_commentary = share_commentary.to_string();
        self.visibility = visibility.to_string();
    }
}

impl Destination for LinkedIn {
    async fn fire(&self, url: &str, tags: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        let mut share_commentary = self.share_commentary.clone();

        if !tags.is_empty() {
            let tags_joined = tags.join(", #");
            share_commentary = format!("{}\n\n#{}", self.share_commentary.clone(), tags_joined);
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
                            "originalUrl": url,
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
