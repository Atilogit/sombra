mod view;

use std::{collections::BTreeMap, time::Duration};

use serde_derive::{Deserialize, Serialize};
use sombra_client::{Battletag, FoundPlayer, Hero, Overbuff, PlayerProfile, Rank, Role, Stat};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub btag: Battletag,
    pub profile: Option<PlayerProfile>,
    pub overbuff: Option<Overbuff>,
    pub found: FoundPlayer,
    pub heroes: Vec<Hero>,
}

pub struct Stats {
    time: Duration,
    win: usize,
    draw: usize,
    loss: usize,
}

impl Player {
    pub async fn fetch(btag: Battletag, heroes: Vec<Hero>) -> Option<Self> {
        None
    }

    pub fn namecard_url(&self) -> String {
        self.found
            .namecard
            .as_ref()
            .map_or(String::new(), std::string::ToString::to_string)
    }

    pub fn title(&self) -> String {
        self.found
            .title
            .as_ref()
            .map_or(String::new(), |t| t["en_US"].clone())
    }

    pub fn ranks(&self) -> Vec<Rank> {
        let mut ranks = self
            .overbuff
            .as_ref()
            .map(|p| &p.ranks)
            .or_else(|| self.profile.as_ref().map(|p| &p.ranks))
            .cloned()
            .unwrap_or(Vec::new());
        ranks.sort_by_key(|r| r.role);
        ranks.reverse();
        ranks
    }

    pub fn rank(&self, role: Role) -> Option<Rank> {
        self.ranks().into_iter().find(|r| r.role == role)
    }

    pub fn role_stats(&self, role: Role) -> Option<Stats> {
        None
    }

    pub fn stats(&self) -> Option<Stats> {
        None
    }

    pub fn hero_stats(&self, hero: &str) -> Option<BTreeMap<String, Stat>> {
        let visible_stats = ["Time Played", "Win Percentage", "Weapon Accuracy"];
        Some(
            self.profile
                .as_ref()?
                .competitive_pc
                .get(hero)?
                .stats
                .iter()
                .filter(|(name, _)| visible_stats.contains(&name.as_str()))
                .map(|(name, stat)| (name.clone(), stat.clone()))
                .collect(),
        )
    }
}
