use serde_derive::{Deserialize, Serialize};

use crate::Rank;

#[cfg_attr(feature = "poem_openapi", derive(poem_openapi::Object))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Overbuff {
    pub ranks: Vec<Rank>,
}
