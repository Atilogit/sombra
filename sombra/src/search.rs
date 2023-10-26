use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use sombra_types::{Battletag, FoundPlayer};
use tracing::instrument;

use crate::{Client, Id};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FoundPlayerRaw {
    pub battle_tag: Battletag,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub last_updated: DateTime<Utc>,
    pub is_public: bool,
    pub frame: Option<Id>,
    pub namecard: Option<Id>,
    pub portrait: Option<Id>,
    pub title: Option<Id>,
}

impl Client {
    #[instrument(level = "debug", skip(self))]
    pub async fn search(&self, name: &str) -> crate::Result<Vec<FoundPlayer>> {
        let url = "https://overwatch.blizzard.com/en-us/search/account-by-name/";
        let raw: Vec<FoundPlayerRaw> =
            serde_json::from_str(&self.get(&format!("{url}{name}")).await?)?;
        Ok(raw
            .into_iter()
            .map(|f| {
                let namecard = f.namecard.and_then(|id| self.assets[&id].icon.clone());
                let portrait = f.portrait.and_then(|id| self.assets[&id].icon.clone());
                let title = f.title.map(|id| self.assets[&id].name.clone());
                FoundPlayer {
                    battle_tag: f.battle_tag,
                    last_updated: f.last_updated,
                    is_public: f.is_public,
                    namecard,
                    portrait,
                    title,
                }
            })
            .collect())
    }
}
