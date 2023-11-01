use std::borrow::Borrow;

use sombra_types::{Color, Hero, Role};
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
                color: hero_color(&name),
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

#[allow(clippy::non_ascii_literal)]
#[allow(clippy::match_same_arms)]
pub fn hero_color(hero: &str) -> Color {
    match hero {
        "D.Va" => "#fc79bdff".parse().unwrap(),
        "Soldier: 76" => "#445275ff".parse().unwrap(),
        "Zarya" => "#f65ea6ff".parse().unwrap(),
        "Widowmaker" => "#8b3f8fff".parse().unwrap(),
        "Hanzo" => "#b2a865ff".parse().unwrap(),
        "Junkrat" => "#f7b217ff".parse().unwrap(),
        "Ana" => "#48699eff".parse().unwrap(),
        "Orisa" => "#106f04ff".parse().unwrap(),
        "Roadhog" => "#ae6f1cff".parse().unwrap(),
        "Mercy" => "#faf2adff".parse().unwrap(),
        "Zenyatta" => "#fcee5aff".parse().unwrap(),
        "Brigitte" => "#72332aff".parse().unwrap(),
        "Genji" => "#80fb00ff".parse().unwrap(),
        "Moira" => "#804be5ff".parse().unwrap(),
        "Bastion" => "#5b7351ff".parse().unwrap(),
        "Pharah" => "#58bcff".parse().unwrap(),
        "Cassidy" => "#a62927ff".parse().unwrap(),
        "Winston" => "#8f92aeff".parse().unwrap(),
        "Illari" => "#a58c54ff".parse().unwrap(),
        "Tracer" => "#de7a00ff".parse().unwrap(),
        "Doomfist" => "#661e0fff".parse().unwrap(),
        "Reinhardt" => "#7c8b8cff".parse().unwrap(),
        "Wrecking Ball" => "#e2790aff".parse().unwrap(),
        "Mei" => "#469af0ff".parse().unwrap(),
        "Lúcio" => "#67c519ff".parse().unwrap(),
        "Torbjörn" => "#ba4c3fff".parse().unwrap(),
        "Sombra" => "#5128a9ff".parse().unwrap(),
        "Symmetra" => "#76b4c9ff".parse().unwrap(),
        "Reaper" => "#5e001aff".parse().unwrap(),
        "Sigma" => "#7c8b8cff".parse().unwrap(),
        "Kiriko" => "#d04656ff".parse().unwrap(),
        "Baptiste" => "#28a5c3ff".parse().unwrap(),
        "Junker Queen" => "#579fcfff".parse().unwrap(),
        "Sojourn" => "#d73e2cff".parse().unwrap(),
        "Ashe" => "#3e3c3aff".parse().unwrap(),
        "Ramattra" => "#7d55c7ff".parse().unwrap(),
        "Echo" => "#89c8ffff".parse().unwrap(),
        "Lifeweaver" => "#e1a5baff".parse().unwrap(),
        _ => "".parse().unwrap(),
    }
}
