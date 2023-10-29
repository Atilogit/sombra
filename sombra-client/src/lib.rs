#![allow(clippy::missing_errors_doc)]

mod error;

pub use error::*;
pub use sombra_types::*;

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

    pub async fn search(&self, name: &str) -> Result<Vec<FoundPlayer>> {
        let url = format!("{}/api/v1/search", self.url);
        let response = self.client.get(url).query(&[("name", name)]).send().await?;
        Error::result_from_status(response.status(), None)?;
        Ok(serde_json::from_str(&response.text().await?)?)
    }

    pub async fn profile(&self, btag: &Battletag) -> Result<PlayerProfileReduced> {
        let url = format!("{}/api/v1/profile", self.url);
        let response = self
            .client
            .get(url)
            .query(&[("name", &btag.name), ("number", &btag.number.to_string())])
            .send()
            .await?;
        Error::result_from_status(response.status(), None)?;
        Ok(serde_json::from_str(&response.text().await?)?)
    }

    pub async fn profile_full(&self, btag: &Battletag) -> Result<PlayerProfile> {
        let url = format!("{}/api/v1/profile_full", self.url);
        let response = self
            .client
            .get(url)
            .query(&[("name", &btag.name), ("number", &btag.number.to_string())])
            .send()
            .await?;
        Error::result_from_status(response.status(), None)?;
        Ok(serde_json::from_str(&response.text().await?)?)
    }

    pub async fn overbuff(&self, btag: &Battletag) -> Result<Overbuff> {
        let url = format!("{}/api/v1/overbuff", self.url);
        let response = self
            .client
            .get(url)
            .query(&[("name", &btag.name), ("number", &btag.number.to_string())])
            .send()
            .await?;
        Error::result_from_status(response.status(), None)?;
        Ok(serde_json::from_str(&response.text().await?)?)
    }

    pub async fn heroes(&self) -> Result<Vec<Hero>> {
        let url = format!("{}/api/v1/heroes", self.url);
        let response = self.client.get(url).send().await?;
        Error::result_from_status(response.status(), None)?;
        Ok(serde_json::from_str(&response.text().await?)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_all() {
        let client = Client::new("http://localhost:8000");
        let found = client.search("player").await.unwrap();
        assert!(!found.is_empty());
        client.profile(&found[0].battle_tag).await.unwrap();
        client.profile_full(&found[0].battle_tag).await.unwrap();
        client.overbuff(&found[0].battle_tag).await.unwrap();
        client.heroes().await.unwrap();
    }
}
