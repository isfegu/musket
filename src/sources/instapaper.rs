use super::{Bookmark, SourceError};
use oauth1::Token;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;
use std::collections::HashMap;
use tracing::debug;

#[derive(Default, Serialize, Deserialize)]
pub struct InstapaperConfiguration {
    pub username: String,
    pub password: String,
    pub consumer_key: String,
    pub consumer_secret: String,
}
pub struct Instapaper {
    pub configuration: InstapaperConfiguration,
}

impl From<reqwest::Error> for SourceError {
    fn from(e: reqwest::Error) -> Self {
        SourceError::Instapaper {
            message: format!("Something went wrong when connecting to Instapaper {e}."),
        }
    }
}

impl From<serde_json::Error> for SourceError {
    fn from(e: serde_json::Error) -> Self {
        SourceError::Instapaper {
            message: format!("Something went wrong when parsing the JSON response {e}."),
        }
    }
}

impl Instapaper {
    pub fn new(username: &str, password: &str, consumer_key: &str, consumer_secret: &str) -> Self {
        Instapaper {
            configuration: InstapaperConfiguration {
                username: username.to_string(),
                password: password.to_string(),
                consumer_key: consumer_key.to_string(),
                consumer_secret: consumer_secret.to_string(),
            },
        }
    }

    pub async fn get_bookmark(&self) -> Result<Bookmark, SourceError> {
        debug!("Inside get_bookmark function");

        let tokens = self.get_oauth_tokens().await?;

        let mut params: HashMap<&str, Cow<str>> = HashMap::new();
        params.insert("limit", Cow::Borrowed("1"));
        params.insert("tag", Cow::Borrowed("musket"));

        let response = Self::do_request(
            "bookmarks/list",
            &self.configuration.consumer_key,
            &self.configuration.consumer_secret,
            &tokens[0],
            &tokens[1],
            Some(params),
        )
        .await?;

        if !response.status().is_success() {
            return Err(SourceError::Instapaper {
                message: "Something went wrong when getting bookmarks from Instapaper.".to_string(),
            });
        }

        let body = response.text().await?;
        let json: Value = serde_json::from_str(&body)?;
        let bookmark = Self::parse_bookmark_response(&json);

        Ok(bookmark)
    }

    pub async fn delete_bookmark(&self, bookmark_id: i64) -> Result<(), SourceError> {
        debug!("Inside delete_bookmark function");

        let tokens = self.get_oauth_tokens().await?;

        let mut params: HashMap<&str, Cow<str>> = HashMap::new();
        params.insert("bookmark_id", Cow::Owned(bookmark_id.to_string()));

        let response = Self::do_request(
            "bookmarks/delete",
            &self.configuration.consumer_key,
            &self.configuration.consumer_secret,
            &tokens[0],
            &tokens[1],
            Some(params),
        )
        .await?;

        if !response.status().is_success() {
            return Err(SourceError::Instapaper {
                message: "Something went wrong when deleting bookmarks from Instapaper."
                    .to_string(),
            });
        }

        Ok(())
    }

    async fn get_oauth_tokens(&self) -> Result<Vec<String>, SourceError> {
        debug!("Inside get_oauth_tokens function");

        let mut params: HashMap<&str, Cow<str>> = HashMap::new();
        params.insert(
            "x_auth_username",
            Cow::Borrowed(&self.configuration.username),
        );
        params.insert(
            "x_auth_password",
            Cow::Borrowed(&self.configuration.password),
        );
        params.insert("x_auth_mode", Cow::Borrowed("client_auth"));

        let response = Self::do_request(
            "oauth/access_token",
            &self.configuration.consumer_key,
            &self.configuration.consumer_secret,
            "",
            "",
            Some(params),
        )
        .await?;

        if !response.status().is_success() {
            return Err(SourceError::Instapaper {
                message: "Something went wrong when getting oauth tokens from Instapaper."
                    .to_string(),
            });
        }

        let mut tokens: Vec<String> = Vec::new();
        let tokens_string = response.text().await?;
        let re = regex::Regex::new(r"oauth_token_secret=([^&]*)&oauth_token=([^&]*)").unwrap();
        if let Some(captures) = re.captures(&tokens_string) {
            let oauth_token_secret = captures.get(1).map_or("", |m| m.as_str());
            let oauth_token = captures.get(2).map_or("", |m| m.as_str());
            tokens.push(oauth_token.to_string());
            tokens.push(oauth_token_secret.to_string());
        }

        Ok(tokens)
    }

    async fn do_request<'a>(
        url: &str,
        consumer_key: &str,
        consumer_secret: &str,
        access_token: &str,
        access_token_secret: &str,
        params: Option<HashMap<&'a str, Cow<'a, str>>>,
    ) -> reqwest::Result<reqwest::Response> {
        debug!("Inside do_request function");

        let request_url = format!("https://www.instapaper.com/api/1/{url}");
        let client = Client::new();
        let response = client
            .post(&request_url)
            .form(&params)
            .header(
                reqwest::header::AUTHORIZATION,
                oauth1::authorize(
                    "POST",
                    &request_url,
                    &Token::new(consumer_key, consumer_secret),
                    Some(&Token::new(access_token, access_token_secret)),
                    params,
                ),
            )
            .send()
            .await?;

        Ok(response)
    }

    fn parse_bookmark_response(json: &Value) -> Bookmark {
        let tags_array = json[2]["tags"].as_array();
        let tags = match tags_array {
            Some(tags) => tags
                .iter()
                .filter(|tag| tag["name"] != "musket")
                .map(|tag| tag["name"].as_str().unwrap_or_default().to_string())
                .collect::<Vec<_>>(),
            None => vec![],
        };

        Bookmark {
            id: json[2]["bookmark_id"].as_i64().unwrap_or_default(),
            url: json[2]["url"].as_str().unwrap_or_default().to_string(),
            tags,
        }
    }
}
