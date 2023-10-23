mod btag;
pub use self::btag::*;

use std::fmt::{Debug, Display};

use chrono::{serde::ts_seconds, DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FoundPlayer {
    pub battle_tag: Battletag,
    #[serde(with = "ts_seconds")]
    pub last_updated: DateTime<Utc>,
    pub is_public: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerProfile {
    pub title: Option<String>,
    pub endorsement: Endorsement,
    pub portrait: Option<String>,
    pub ranks: Vec<Rank>,
}

bounded_integer::bounded_integer! {
    pub struct Endorsement{ 1..=5 }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rank {
    pub group: Group,
    pub division: Division,
    pub role: Role,
    pub console: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Role {
    Tank,
    Damage,
    Support,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum Group {
    Bronze,
    Silver,
    Gold,
    Platinum,
    Diamond,
    Master,
    Grandmaster,
}

bounded_integer::bounded_integer! {
    pub struct Division{ 1..=5 }
}

impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.group, self.division)
    }
}
