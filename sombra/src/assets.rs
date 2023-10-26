use std::collections::HashMap;

use sombra_types::{Asset, Id};
use tracing::instrument;

use crate::{Client, Error};

impl Client {
    #[instrument(level = "debug", skip(self))]
    pub async fn fetch_assets(&mut self) -> crate::Result<()> {
        let html = self
            .get("https://overwatch.blizzard.com/en-us/search/")
            .await?;
        let mut split = html.split("const ").skip(2);

        let avatars: HashMap<Id, Asset> = parse_json_var(split.next().ok_or_else(Error::parse)?)?;
        let namecards: HashMap<Id, Asset> = parse_json_var(split.next().ok_or_else(Error::parse)?)?;
        let titles: HashMap<Id, Asset> = parse_json_var(split.next().ok_or_else(Error::parse)?)?;

        let mut assets = HashMap::new();
        assets.extend(avatars.into_iter());
        assets.extend(namecards.into_iter());
        assets.extend(titles.into_iter());
        self.assets = assets;

        Ok(())
    }
}

#[instrument(level = "debug", skip_all)]
fn parse_json_var<'de, T: serde::Deserialize<'de>>(js: &'de str) -> crate::Result<T> {
    let json = js
        .split('=')
        .nth(1)
        .ok_or_else(Error::parse)?
        .trim()
        .split("</script>")
        .next()
        .ok_or_else(Error::parse)?;
    Ok(serde_json::from_str::<'de>(json)?)
}
