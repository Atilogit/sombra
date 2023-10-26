use serde_derive::{Serialize, Deserialize};

use crate::Rank;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Overbuff {
    pub ranks: Vec<Rank>,
}
