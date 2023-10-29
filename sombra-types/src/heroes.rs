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
}
