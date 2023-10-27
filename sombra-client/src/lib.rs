#![allow(clippy::missing_errors_doc)]

mod error;

pub use error::*;

use sombra_types::FoundPlayer;

pub struct Client {
    url: String,
    client: reqwest::Client,
}

impl Client {
    #[must_use]
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            client: reqwest::Client::new(),
        }
    }

    async fn get(&self, url: &str) -> Result<String> {
        let response = self.client.get(url).send().await?;
        Error::result_from_status(response.status(), None)?;
        Ok(response.text().await?)
    }

    pub async fn search(&self, name: &str) -> Result<Vec<FoundPlayer>> {
        let url = format!("{}/api/v1/search/{}", self.url, name);
        let response = self.get(&url).await?;
        Ok(serde_json::from_str(&response)?)
    }
}
