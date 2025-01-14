use super::{Destination, DestinationError};
use chrono::prelude::*;
use libsql::Builder;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TursoConfiguration {
    pub database: String,
    pub token: String,
    pub enabled: bool,
}
pub struct Turso {
    pub configuration: TursoConfiguration,
    pub url: String,
    pub tags: Vec<String>,
}

impl From<libsql::Error> for DestinationError {
    fn from(e: libsql::Error) -> Self {
        DestinationError::Turso {
            message: format!("The url cannot be sent to Turso due to {e}."),
        }
    }
}

impl Destination for Turso {
    async fn fire(&self) -> Result<(), DestinationError> {
        let local: DateTime<Local> = Local::now();
        let created = format!("{}", local.format("%Y-%m-%d %H:%M:%S"));

        let turso_db_url = self.configuration.database.clone();
        let turso_db_token = self.configuration.token.clone();

        let db = Builder::new_remote(turso_db_url, turso_db_token)
            .build()
            .await?;

        let conn = db.connect()?;

        conn.execute(
            "INSERT INTO links (url, tags, created) VALUES (:url, :tags, :created)",
            libsql::named_params! { ":url": self.url.clone(), ":tags": self.tags.join(", "), ":created": created },
        )
        .await?;

        Ok(())
    }
}
