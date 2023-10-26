use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;
use std::time::Duration;
use url::Url;

use crate::Battletag;

#[cfg_attr(feature = "poem_openapi", derive(poem_openapi::Object))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerProfile {
    pub battletag: Battletag,
    pub title: Option<String>,
    pub endorsement: Endorsement,
    pub portrait: Url,
    pub ranks: Vec<Rank>,
    pub private: bool,
    pub last_updated: DateTime<Utc>,
    pub quickplay_console: HashMap<String, HeroStats>,
    pub competitive_console: HashMap<String, HeroStats>,
    pub quickplay_pc: HashMap<String, HeroStats>,
    pub competitive_pc: HashMap<String, HeroStats>,
}

#[cfg_attr(feature = "poem_openapi", derive(poem_openapi::Object))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerProfileReduced {
    pub battletag: Battletag,
    pub title: Option<String>,
    pub endorsement: Endorsement,
    pub portrait: Url,
    pub ranks: Vec<Rank>,
    pub private: bool,
    pub last_updated: DateTime<Utc>,
}

bounded_integer::bounded_integer! {
    #[cfg_attr(feature = "poem_openapi", derive(poem_openapi::NewType))]
    pub struct Endorsement{ 1..=5 }
}

#[cfg_attr(feature = "poem_openapi", derive(poem_openapi::Object))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rank {
    pub group: Group,
    pub division: Division,
    pub role: Role,
    pub console: bool,
}

#[cfg_attr(feature = "poem_openapi", derive(poem_openapi::Enum))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Role {
    Tank,
    Damage,
    Support,
}

#[cfg_attr(feature = "poem_openapi", derive(poem_openapi::Enum))]
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
    #[cfg_attr(feature = "poem_openapi", derive(poem_openapi::NewType))]
    pub struct Division{ 1..=5 }
}

#[cfg_attr(feature = "poem_openapi", derive(poem_openapi::Object))]
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

impl From<&PlayerProfile> for PlayerProfileReduced {
    fn from(value: &PlayerProfile) -> Self {
        Self {
            battletag: value.battletag.clone(),
            title: value.title.clone(),
            endorsement: value.endorsement,
            portrait: value.portrait.clone(),
            ranks: value.ranks.clone(),
            private: value.private,
            last_updated: value.last_updated,
        }
    }
}

#[cfg(feature = "poem_openapi")]
impl poem_openapi::types::Type for Stat {
    const IS_REQUIRED: bool = true;

    type RawValueType = Self;

    type RawElementValueType = Self;

    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("Stat")
    }

    fn schema_ref() -> poem_openapi::registry::MetaSchemaRef {
        poem_openapi::registry::MetaSchemaRef::Reference(Self::name().into_owned())
    }

    fn register(registry: &mut poem_openapi::registry::Registry) {
        use poem_openapi::types::ToJSON;
        registry.create_schema::<Self, _>(Self::name().into_owned(), |_| {
            poem_openapi::registry::MetaSchema {
                description: None,
                external_docs: None,
                deprecated: false,
                enum_items: vec![
                    Self::Number(0.).to_json().unwrap(),
                    Self::Duration(Duration::ZERO).to_json().unwrap(),
                    Self::Percentage(0.).to_json().unwrap(),
                ],
                ..poem_openapi::registry::MetaSchema::new("string")
            }
        });
    }

    fn as_raw_value(&self) -> Option<&Self::RawValueType> {
        Some(self)
    }

    fn raw_element_iter<'se>(
        &'se self,
    ) -> Box<dyn Iterator<Item = &'se Self::RawElementValueType> + 'se> {
        Box::new(
            [
                Self::Number(0.),
                Self::Duration(Duration::ZERO),
                Self::Percentage(0.),
            ]
            .iter(),
        )
    }
}

#[cfg(feature = "poem_openapi")]
impl poem_openapi::types::ParseFromJSON for Stat {
    fn parse_from_json(value: Option<serde_json::Value>) -> poem_openapi::types::ParseResult<Self> {
        let value = value.ok_or_else(poem_openapi::types::ParseError::expected_input)?;
        match value {
            serde_json::Value::Number(n) => Ok(Self::Number(n.as_f64().unwrap())),
            serde_json::Value::String(s) => Ok(s.parse().unwrap()),
            _ => Err(poem_openapi::types::ParseError::expected_type(value)),
        }
    }
}

#[cfg(feature = "poem_openapi")]
impl poem_openapi::types::ToJSON for Stat {
    fn to_json(&self) -> Option<serde_json::Value> {
        Some(serde_json::Value::String(self.to_string()))
    }
}

impl FromStr for Stat {
    type Err = ();

    #[allow(clippy::map_err_ignore)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with('%') {
            Ok(Self::Percentage(
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

impl Display for Stat {
    #[allow(clippy::many_single_char_names)]
    #[allow(clippy::integer_division)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::Duration(d) => {
                let mut d = *d;
                let h = d.as_secs() / 60 / 60;
                d -= Duration::from_secs(h * 60 * 60);
                let m = d.as_secs() / 60;
                d -= Duration::from_secs(m * 60);
                let s = d.as_secs();
                if h > 0 {
                    write!(f, "{h}:{m:02}:{s:02}")
                } else {
                    write!(f, "{m}:{s:02}")
                }
            }
            Self::Percentage(p) => write!(f, "{p}%"),
        }
    }
}
