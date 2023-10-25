use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;
use std::time::Duration;
use url::Url;

use crate::Battletag;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerProfile {
    pub battletag: Battletag,
    pub title: Option<String>,
    pub endorsement: Endorsement,
    pub portrait: Url,
    pub ranks: Vec<Rank>,
    pub private: bool,
    pub quickplay_console: HashMap<String, HeroStats>,
    pub competitive_console: HashMap<String, HeroStats>,
    pub quickplay_pc: HashMap<String, HeroStats>,
    pub competitive_pc: HashMap<String, HeroStats>,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeroStats {
    pub stats: HashMap<String, Stat>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Stat {
    Number(f64),
    Duration(Duration),
    Percentage(f64),
}

impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.group, self.division)
    }
}

impl FromStr for Stat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with('%') {
            Ok(Self::Number(
                s.trim_end_matches('%').parse().map_err(|_| ())?,
            ))
        } else if s.contains(':') {
            let mut split = s.split(':');
            match split.clone().count() {
                2 => {
                    let m: u64 = split.next().unwrap().parse().map_err(|_| ())?;
                    let s: u64 = split.next().unwrap().parse().map_err(|_| ())?;
                    Ok(Self::Duration(Duration::from_secs(m * 60 + s)))
                }
                3 => {
                    let h: u64 = split.next().unwrap().parse().map_err(|_| ())?;
                    let m: u64 = split.next().unwrap().parse().map_err(|_| ())?;
                    let s: u64 = split.next().unwrap().parse().map_err(|_| ())?;
                    Ok(Self::Duration(Duration::from_secs(
                        h * 60 * 60 + m * 60 + s,
                    )))
                }
                _ => Err(()),
            }
        } else {
            Ok(Self::Number(s.parse().map_err(|_| ())?))
        }
    }
}
