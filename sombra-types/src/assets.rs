use std::fmt::Debug;
use std::str::FromStr;
use std::{collections::HashMap, fmt::Display};

use serde_derive::{Deserialize, Serialize};
use url::Url;

#[cfg_attr(feature = "poem_openapi", derive(poem_openapi::NewType))]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "String")]
#[serde(into = "u64")]
pub struct Id(u64);

#[cfg_attr(feature = "poem_openapi", derive(poem_openapi::Object))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Asset {
    pub id: Id,
    pub name: HashMap<String, String>,
    #[serde(rename = "type")]
    pub typ: IdName,
    pub rarity: RarityTypes,
    pub hero: IdName,
    pub release: Release,
    pub event: IdName,
    pub is_new: bool,
    pub is_marked: bool,
    pub data: Data,
    pub icon: Option<Url>,
}

#[cfg_attr(feature = "poem_openapi", derive(poem_openapi::Object))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Data {
    pub category: Category,
    pub thumbnail: String,
    pub name: HashMap<String, String>,
    pub description: String,
    #[serde(rename = "type")]
    pub typ: Option<ContentType>,
    pub event: IdName,
    pub release: Release,
    pub rarity: Rarity,
    #[serde(deserialize_with = "option_string")]
    pub url: Option<String>,
    pub video_webm: bool,
    pub video_mp4: bool,
}

#[cfg_attr(feature = "poem_openapi", derive(poem_openapi::Object))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IdName {
    pub id: Option<Id>,
    pub name: Option<String>,
}

#[cfg_attr(feature = "poem_openapi", derive(poem_openapi::Object))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Rarity {
    pub name: RarityTypes,
    pub value: i64,
}

#[cfg_attr(feature = "poem_openapi", derive(poem_openapi::Enum))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(deny_unknown_fields)]
pub enum RarityTypes {
    Common,
    Epic,
    Rare,
}

#[cfg_attr(feature = "poem_openapi", derive(poem_openapi::Object))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Release {
    pub id: Id,
    pub name: String,
    pub version: f64,
    pub title: Option<String>,
}

#[cfg_attr(feature = "poem_openapi", derive(poem_openapi::Enum))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum Category {
    Avatars,
    Namecards,
    Titles,
}

#[cfg_attr(feature = "poem_openapi", derive(poem_openapi::Enum))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum ContentType {
    Image,
}

#[allow(clippy::unnecessary_wraps)]
fn option_string<'de, D>(de: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct Visitor;

    impl serde::de::Visitor<'_> for Visitor {
        type Value = String;

        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "string or false")
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v)
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v.to_owned())
        }
    }

    Ok(de.deserialize_string(Visitor).ok())
}

impl TryFrom<String> for Id {
    type Error = std::num::ParseIntError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

impl TryFrom<&str> for Id {
    type Error = std::num::ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(u64::from_str_radix(
            value.trim_start_matches("0x"),
            16,
        )?))
    }
}

impl From<Id> for u64 {
    fn from(val: Id) -> Self {
        val.0
    }
}

impl Debug for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl FromStr for Id {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.try_into()
    }
}
