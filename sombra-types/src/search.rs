use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use url::Url;

use crate::Battletag;

#[cfg_attr(feature = "poem_openapi", derive(poem_openapi::Object))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "poem_openapi", oai(rename_all = "camelCase"))]
#[serde(rename_all = "camelCase")]
pub struct FoundPlayer {
    pub battle_tag: Battletag,
    pub last_updated: DateTime<Utc>,
    pub is_public: bool,
    pub namecard: Option<Url>,
    pub portrait: Option<Url>,
    pub title: Option<HashMap<String, String>>,
}
