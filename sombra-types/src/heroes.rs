use std::{fmt::Display, str::FromStr};

use serde_derive::{Deserialize, Serialize};
use url::Url;

use crate::Role;

#[cfg_attr(feature = "poem_openapi", derive(poem_openapi::Object))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "poem_openapi", oai(rename_all = "camelCase"))]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Hero {
    pub name: String,
    pub role: Role,
    pub portrait: Url,
    pub color: Color,
}

#[cfg_attr(feature = "poem_openapi", derive(poem_openapi::Object))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "poem_openapi", oai(rename_all = "camelCase"))]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl FromStr for Color {
    type Err = ();

    #[allow(clippy::string_slice)]
    #[allow(clippy::map_err_ignore)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_start_matches('#');
        let r = u8::from_str_radix(&s[0..2], 16).map_err(|_| ())?;
        let g = u8::from_str_radix(&s[2..4], 16).map_err(|_| ())?;
        let b = u8::from_str_radix(&s[4..6], 16).map_err(|_| ())?;
        Ok(Self { r, g, b })
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}
