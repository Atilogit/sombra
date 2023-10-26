#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::map_err_ignore)]

mod assets;
mod cached;
mod error;
mod overbuff;
mod profile;
mod search;
mod util;

use std::collections::HashMap;

pub use assets::*;
pub use cached::*;
pub use error::*;
pub use overbuff::*;
pub use profile::*;
pub use search::*;
pub use sombra_types::*;

use tracing::instrument;

#[derive(Debug, Clone)]
pub struct Client {
    client: reqwest::Client,
    assets: HashMap<Id, Asset>,
}

impl Client {
    pub async fn new() -> Result<Self> {
        let client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36")
            .build()
            .expect("Could not build client");
        let mut s = Self {
            client,
            assets: HashMap::new(),
        };
        s.fetch_assets().await?;
        Ok(s)
    }

    #[instrument(level = "debug", skip(self))]
    async fn get(&self, url: &str) -> Result<String> {
        let response = self.client.get(url).send().await?;
        Error::result_from_status(response.status(), None)?;
        Ok(response.text().await?)
    }

    #[must_use]
    pub const fn assets(&self) -> &HashMap<Id, Asset> {
        &self.assets
    }
}
