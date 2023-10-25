use chrono::{serde::ts_seconds, DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use tracing::instrument;

use crate::{Battletag, Client, Id};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FoundPlayer {
    pub battle_tag: Battletag,
    #[serde(with = "ts_seconds")]
    pub last_updated: DateTime<Utc>,
    pub is_public: bool,
    pub frame: Option<Id>,
    pub namecard: Option<Id>,
    pub portrait: Option<Id>,
    pub title: Option<Id>,
}

impl Client {
    #[instrument(skip(self))]
    pub async fn search(&self, name: &str) -> crate::Result<Vec<FoundPlayer>> {
        let url = "https://overwatch.blizzard.com/en-us/search/account-by-name/";
        Ok(serde_json::from_str(
            &self.get(&format!("{url}{name}")).await?,
        )?)
    }
}
