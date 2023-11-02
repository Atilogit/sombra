mod view;

use std::{collections::BTreeMap, time::Duration};

use serde_derive::{Deserialize, Serialize};
use sombra_client::{
    Battletag, Client, FoundPlayer, Hero, Overbuff, PlayerProfile, Rank, Role, Stat,
};

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
    pub async fn fetch(btag: Battletag, heroes: Vec<Hero>, client: Client) -> Option<Self> {
        let found = client
            .search(&btag.name)
            .await
            .ok()
            .and_then(|v| v.into_iter().find(|p| p.battle_tag == btag))?;
        let profile = client.profile_full(&btag).await.ok();
        let fetch_overbuff = profile.is_none()
            || !found.is_public
            || profile.as_ref().is_some_and(|p| p.ranks.is_empty());
        let overbuff = if fetch_overbuff {
            client.overbuff(&btag).await.ok()
        } else {
            None
        };
        Some(Self {
            btag,
            profile,
            overbuff,
            found,
            heroes,
        })
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

    #[allow(clippy::cast_sign_loss)]
    pub fn role_stats(&self, role: Role) -> Option<Stats> {
        let all = &self.profile.as_ref()?.competitive_pc;
        let heroes = self
            .heroes
            .iter()
            .filter(|h| h.role == role)
            .filter_map(|hero| all.get(&hero.name));
        let time = heroes
            .clone()
            .filter_map(|stats| stats.stats.get("Time Played"))
            .copied()
            .sum::<Option<Stat>>()
            .unwrap_or(Stat::Duration(Duration::ZERO))
            .as_duration()?;
        let win = heroes
            .clone()
            .filter_map(|stats| stats.stats.get("Games Won"))
            .copied()
            .sum::<Option<Stat>>()
            .unwrap_or(Stat::Number(0.))
            .as_f64()? as usize;
        let draw = heroes
            .clone()
            .filter_map(|stats| stats.stats.get("Games Tied"))
            .copied()
            .sum::<Option<Stat>>()
            .unwrap_or(Stat::Number(0.))
            .as_f64()? as usize;
        let loss = heroes
            .filter_map(|stats| stats.stats.get("Games Lost"))
            .copied()
            .sum::<Option<Stat>>()
            .unwrap_or(Stat::Number(0.))
            .as_f64()? as usize;
        Some(Stats {
            time,
            win,
            draw,
            loss,
        })
    }

    #[allow(clippy::cast_sign_loss)]
    pub fn stats(&self) -> Option<Stats> {
        let all = &self
            .profile
            .as_ref()?
            .competitive_pc
            .get("ALL HEROES")?
            .stats;
        let time = all
            .get("Time Played")
            .unwrap_or(&Stat::Duration(Duration::ZERO))
            .as_duration()?;
        let win = all.get("Games Won").unwrap_or(&Stat::Number(0.)).as_f64()? as usize;
        let draw = all
            .get("Games Tied")
            .unwrap_or(&Stat::Number(0.))
            .as_f64()? as usize;
        let loss = all
            .get("Games Lost")
            .unwrap_or(&Stat::Number(0.))
            .as_f64()? as usize;
        Some(Stats {
            time,
            win,
            draw,
            loss,
        })
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
                .map(|(name, stat)| (name.clone(), *stat))
                .collect(),
        )
    }
}
