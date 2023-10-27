use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use serde_derive::{Deserialize, Serialize};

#[cfg_attr(feature = "poem_openapi", derive(poem_openapi::Object))]
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "poem_openapi", oai(rename_all = "camelCase"))]
#[serde(rename_all = "camelCase")]
pub struct Battletag {
    pub name: String,
    pub number: u64,
}

impl Battletag {
    pub fn new<S: Into<String>>(name: S, number: u64) -> Self {
        Self {
            name: name.into(),
            number,
        }
    }
}

impl Debug for Battletag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
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

impl FromStr for Battletag {
    type Err = ();

    #[allow(clippy::map_err_ignore)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, number) = s.split_once('#').or_else(|| s.split_once('-')).ok_or(())?;
        let number = number.parse().map_err(|_| ())?;
        Ok(Self {
            name: name.to_owned(),
            number,
        })
    }
}

impl TryFrom<String> for Battletag {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse().map_err(|()| s)
    }
}

impl From<Battletag> for String {
    fn from(val: Battletag) -> Self {
        val.to_string()
    }
}
