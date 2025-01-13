use super::{Destination, DestinationError};
use megalodon::{entities::StatusVisibility, generator, megalodon::PostStatusInputOptions};

pub struct Mastodon {
    pub server: String,
    pub token: String,
    pub url: String,
    pub tags: Vec<String>,
    pub commentary: String,
}

impl From<megalodon::error::Error> for DestinationError {
    fn from(e: megalodon::error::Error) -> Self {
        DestinationError::Mastodon {
            message: format!("The url cannot be sent to Mastodon due to {e}."),
        }
    }
}

impl Destination for Mastodon {
    async fn fire(&self) -> Result<(), DestinationError> {
        match generator(
            megalodon::SNS::Mastodon,
            self.server.clone(),
            Some(self.token.clone()),
            Some(String::from("Musket")),
        ) {
            Err(e) => {
                return Err(DestinationError::Mastodon {
                    message: format!("The url cannot be sent to Mastodon due to {e}."),
                });
            }
            Ok(mastodon_client) => {
                let mut status_content = format!("{}\n{}", self.commentary, self.url);

                if !self.tags.is_empty() {
                    let tags_joined = self.tags.join(", #");
                    status_content = format!("{}\n#{}", status_content, &tags_joined);
                }

                mastodon_client
                    .post_status(
                        status_content,
                        Some(&PostStatusInputOptions {
                            visibility: Some(StatusVisibility::Public),
                            language: Some("es".to_string()),
                            ..Default::default()
                        }),
                    )
                    .await?;
            }
        }

        Ok(())
    }
}
