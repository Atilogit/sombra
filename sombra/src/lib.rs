#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]

mod error;
mod types;

pub use error::*;
pub use types::*;

pub struct Client {
    client: reqwest::Client,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    #[must_use]
    pub fn new() -> Self {
        Self { client: reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36")
        .build()
        .expect("Could not build client") }
    }

    async fn get(&self, url: String) -> Result<String> {
        let response = self.client.get(url).send().await?;
        Error::result_from_status(response.status(), None)?;
        Ok(response.text().await?)
    }

    pub async fn search(&self, name: &str) -> Result<Vec<FoundPlayer>> {
        let url = "https://overwatch.blizzard.com/en-us/search/account-by-name/";
        Ok(serde_json::from_str(
            &self.get(format!("{url}{name}")).await?,
        )?)
    }

    pub async fn profile(&self, btag: &Battletag) -> Result<PlayerProfile> {
        todo!()
    }
}
