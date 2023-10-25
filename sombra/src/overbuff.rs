use tracing::instrument;

use crate::{Battletag, Client};

impl Client {
    #[instrument(skip(self))]
    pub async fn profile_overbuff(&self, _btag: &Battletag) -> crate::Result<()> {
        todo!()
    }
}
