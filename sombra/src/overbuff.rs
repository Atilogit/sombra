use tracing::instrument;

use crate::{Battletag, Client};

impl Client {
    #[instrument(level = "debug", skip(self))]
    pub async fn overbuff(&self, _btag: &Battletag) -> crate::Result<()> {
        todo!()
    }
}
