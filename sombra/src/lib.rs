#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]

mod assets;
mod btag;
mod error;
mod overbuff;
mod profile;
mod search;

pub use assets::*;
pub use btag::*;
pub use error::*;
pub use overbuff::*;
pub use profile::*;
pub use search::*;

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

    async fn get(&self, url: &str) -> crate::Result<String> {
        let response = self.client.get(url).send().await?;
        Error::result_from_status(response.status(), None)?;
        Ok(response.text().await?)
    }
}
