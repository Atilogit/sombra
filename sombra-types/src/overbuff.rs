use poem_openapi::Object;
use serde_derive::{Deserialize, Serialize};

use crate::Rank;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Object)]
pub struct Overbuff {
    pub ranks: Vec<Rank>,
}
