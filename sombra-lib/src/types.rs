use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FoundPlayer {
    #[serde(with = "btag_from_string")]
    pub battle_tag: Battletag,
    pub last_updated: u64,
    pub is_public: bool,
    pub portrait: String,
    pub frame: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerProfile {
    pub title: Option<String>,
    pub endorsement: Limited<5>,
    pub portrait: Option<String>,
    pub ranks: Vec<Rank>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rank {
    pub group: Group,
    pub tier: Limited<5>,
    pub role: Role,
    pub console: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Role {
    Tank,
    Damage,
    Support,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Group {
    Bronze,
    Silver,
    Gold,
    Platinum,
    Diamond,
    Master,
    Grandmaster,
}

impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.group, self.tier)
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct Limited<const MAX: u64>(u64);

impl<const MAX: u64> Debug for Limited<MAX> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const MAX: u64> Display for Limited<MAX> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const MAX: u64> Limited<MAX> {
    pub fn inner(self) -> u64 {
        self.0
    }
}

impl<const MAX: u64> FromStr for Limited<MAX> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val: u64 = s.parse().map_err(|_| ())?;
        val.try_into().map_err(|_| ())
    }
}

impl<const MAX: u64> TryFrom<u64> for Limited<MAX> {
    type Error = u64;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value <= MAX {
            Ok(Limited(value))
        } else {
            Err(value)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Battletag {
    pub name: String,
    pub number: u64,
}

impl Display for Battletag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{}-{}", self.name, self.number)
        } else {
            write!(f, "{}#{}", self.name, self.number)
        }
    }
}

impl Battletag {
    pub fn new<S: Into<String>>(name: S, number: u64) -> Self {
        Self {
            name: name.into(),
            number,
        }
    }
}

mod btag_from_string {
    use crate::Battletag;

    pub fn serialize<S: serde::Serializer>(
        btag: &super::Battletag,
        ser: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        ser.serialize_str(&format!("{btag}"))
    }

    pub fn deserialize<'a, D: serde::Deserializer<'a>>(
        de: D,
    ) -> std::result::Result<super::Battletag, D::Error> {
        struct Visitor;

        impl serde::de::Visitor<'_> for Visitor {
            type Value = Battletag;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "<name>#<number>")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let (name, number) = v.split_once('#').ok_or(E::custom("no # found"))?;
                let number = number.parse().map_err(|e| E::custom(e))?;
                Ok(Battletag {
                    name: name.to_owned(),
                    number,
                })
            }
        }

        de.deserialize_str(Visitor)
    }
}
