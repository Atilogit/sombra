use std::{collections::HashMap, fmt::Debug, hash::Hash};

use serde_derive::{Deserialize, Serialize};

use crate::{Client, Error};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "String")]
#[serde(into = "u64")]
pub struct Id(u64);

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
    pub icon: Option<String>,
}

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
    pub url: OptionString,
    pub video_webm: bool,
    pub video_mp4: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
pub enum OptionString {
    Some(String),
    None(bool),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IdName {
    pub id: Option<Id>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Rarity {
    pub name: RarityTypes,
    pub value: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(deny_unknown_fields)]
pub enum RarityTypes {
    Common,
    Epic,
    Rare,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Release {
    pub id: Id,
    pub name: String,
    pub version: f64,
    pub title: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum Category {
    Avatars,
    Namecards,
    Titles,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum ContentType {
    Image,
}

impl Client {
    pub async fn assets(&self) -> crate::Result<HashMap<Id, Asset>> {
        let html = self
            .get("https://overwatch.blizzard.com/en-us/search/")
            .await?;
        let mut split = html.split("const ").skip(2);

        let avatars: HashMap<Id, Asset> = parse_json_var(split.next().ok_or(Error::Parse)?)?;
        let namecards: HashMap<Id, Asset> = parse_json_var(split.next().ok_or(Error::Parse)?)?;
        let titles: HashMap<Id, Asset> = parse_json_var(split.next().ok_or(Error::Parse)?)?;

        let mut assets = HashMap::new();
        assets.extend(avatars.into_iter());
        assets.extend(namecards.into_iter());
        assets.extend(titles.into_iter());

        Ok(assets)
    }
}

fn parse_json_var<'de, T: serde::Deserialize<'de>>(js: &'de str) -> crate::Result<T> {
    let json = js
        .split('=')
        .nth(1)
        .ok_or(Error::Parse)?
        .trim()
        .split("</script>")
        .next()
        .ok_or(Error::Parse)?;
    Ok(serde_json::from_str::<'de>(json)?)
}

impl TryFrom<String> for Id {
    type Error = std::num::ParseIntError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
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

impl From<OptionString> for Option<String> {
    fn from(val: OptionString) -> Self {
        match val {
            OptionString::Some(s) => Some(s),
            OptionString::None(false) => None,
            OptionString::None(true) => unreachable!(),
        }
    }
}

impl Debug for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}
