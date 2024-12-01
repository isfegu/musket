use super::Destination;
use chrono::prelude::*;
use libsql::Builder;

pub struct Turso {
    pub url: String,
    pub token: String,
}

impl Destination for Turso {
    async fn fire(&self, url: &str, tags: &[String]) -> Result<(), String> {
        let local: DateTime<Local> = Local::now();
        let created = format!("{}", local.format("%Y-%m-%d %H:%M:%S"));

        let turso_db_url = self.url.clone();
        let turso_db_token = self.token.clone();

        let db = Builder::new_remote(turso_db_url, turso_db_token)
            .build()
            .await
            .map_err(|err| format!("{}.", err))?;

        let conn = db.connect().map_err(|err| format!("{}.", err))?;

        conn.execute(
            "INSERT INTO links (url, tags, created) VALUES (:url, :tags, :created)",
            libsql::named_params! { ":url": url, ":tags": tags.join(", "), ":created": created },
        )
        .await
        .map_err(|err| format!("{}.", err))?;

        Ok(())
    }
}
