#![forbid(clippy::unwrap_used)]

mod error;
mod profile;
mod types;

use std::time::Instant;

pub use error::*;
use tl::ParserOptions;
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
    pub fn new() -> Self {
        Self { client: reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36")
        .build()
        .expect("Could not build reqwest client") }
    }

    async fn get(&self, url: String) -> Result<String> {
        let response = self.client.get(url).send().await?;
        Error::result_from_status(response.status(), None)?;
        Ok(response.text().await?)
    }

    pub async fn accounts_by_name(&self, name: &str) -> Result<Vec<FoundPlayer>> {
        let url = "https://overwatch.blizzard.com/en-us/search/account-by-name/";
        Ok(serde_json::from_str(
            &self.get(format!("{url}{name}")).await?,
        )?)
    }

    pub async fn profile(&self, btag: &Battletag) -> Result<PlayerProfile> {
        let url = "https://overwatch.blizzard.com/en-us/career/";
        let html = self.get(format!("{url}{btag:#}/")).await?;
        let html = tl::parse(&html, ParserOptions::new())?;

        let title = profile::find_by_class(&html, "Profile-player--title");

        let endorsement_url =
            profile::tag_content_by_class(&html, "Profile-playerSummary--endorsement", "src")?
                .ok_or(Error::parse("Unable to find endorsement level"))?;

        let endorsement = profile::url_file_name(endorsement_url)
            .ok_or(Error::parse("Invalid endorsement url"))?[..1]
            .parse()
            .map_err(|_| Error::parse("Invalid endorsement url"))?;

        let portrait = profile::tag_content_by_class(&html, "Profile-player--portrait", "src")
            .ok()
            .flatten()
            .map(|s| s.to_owned());

        let ranks = profile::parse_ranks(&html)?;

        Ok(PlayerProfile {
            title,
            endorsement,
            portrait,
            ranks,
        })
    }
}
