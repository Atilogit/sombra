use std::borrow::Borrow;

use sombra_types::{Hero, Role};
use tl::ParserOptions;
use tracing::instrument;

use crate::{
    util::{find_all, find_attr2},
    Client, Error,
};

impl Client {
    #[instrument(level = "debug", skip(self))]
    pub async fn fetch_heroes(&mut self) -> crate::Result<()> {
        let mut heroes = Vec::new();
        let html = self
            .get("https://overwatch.blizzard.com/en-us/heroes/")
            .await?;
        let dom = tl::parse(&html, ParserOptions::new())?;

        for card in find_all(&dom, ".heroCard") {
            let portrait = find_attr2(&dom, card, ".heroCardPortrait", "src")
                .ok_or_else(Error::parse)?
                .parse()
                .map_err(|_| Error::parse())?;
            let role_str = card
                .attributes()
                .get("data-role")
                .flatten()
                .ok_or_else(Error::parse)?
                .as_utf8_str();
            let role = match role_str.borrow() {
                "tank" => Role::Tank,
                "damage" => Role::Damage,
                "support" => Role::Support,
                _ => return Err(Error::parse()),
            };
            let name = card
                .attributes()
                .get("hero-name")
                .flatten()
                .ok_or_else(Error::parse)?
                .as_utf8_str()
                .to_string();

            heroes.push(Hero {
                name,
                role,
                portrait,
            });
        }

        self.heroes = heroes;
        Ok(())
    }

    #[must_use]
    pub fn heroes(&self) -> &[Hero] {
        &self.heroes
    }
}
