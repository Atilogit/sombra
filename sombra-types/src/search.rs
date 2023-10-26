use std::collections::HashMap;

use chrono::{serde::ts_seconds, DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use url::Url;

use crate::Battletag;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FoundPlayer {
    pub battle_tag: Battletag,
    #[serde(with = "ts_seconds")]
    pub last_updated: DateTime<Utc>,
    pub is_public: bool,
    pub namecard: Option<Url>,
    pub portrait: Option<Url>,
    pub title: Option<HashMap<String, String>>,
}
