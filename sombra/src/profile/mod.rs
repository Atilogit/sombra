mod types;
mod util;

use self::util::{find, find2, find_all, find_all2, find_attr2, find_inner_text2};
use crate::{Battletag, Client, Error};
use std::collections::HashMap;
use tl::{ParserOptions, VDom};
pub use types::*;
use url::Url;
use util::{find_attr, find_inner_text, url_file};

impl Client {
    pub async fn profile(&self, btag: Battletag) -> crate::Result<PlayerProfile> {
        let url = "https://overwatch.blizzard.com/en-us/career/";
        let html = self.get(&format!("{url}{btag:#}/")).await?;
        let dom = tl::parse(&html, ParserOptions::new())?;

        let public = find(&dom, ".Profile-player--privateText").is_none();
        let (quickplay_stats, competitive_stats);
        if public {
            quickplay_stats = hero_summary(&dom, "quickPlay-view")?;
            competitive_stats = hero_summary(&dom, "competitive-view")?;
        } else {
            quickplay_stats = HashMap::new();
            competitive_stats = HashMap::new();
        }

        Ok(PlayerProfile {
            battletag: btag,
            title: find_inner_text(&dom, ".Profile-player--title"),
            endorsement: endorsement(&dom)?,
            portrait: portrait(&dom)?,
            ranks: ranks(&dom)?,
            private: !public,
            quickplay_stats,
            competitive_stats,
        })
    }
}

fn hero_summary<'dom>(
    dom: &'dom VDom<'dom>,
    view: &str,
) -> crate::Result<HashMap<String, HeroStats>> {
    let mut heroes = HashMap::new();
    let container_selector = format!(".stats.{view}");
    let container = find(dom, &container_selector).ok_or(Error::Parse)?;

    let select = find2(dom, container, ".Profile-dropdown").ok_or(Error::Parse)?;
    let mut stat_ids = HashMap::new();
    for c in find_all2(dom, select, "option") {
        let id = c
            .attributes()
            .get("value")
            .flatten()
            .ok_or(Error::Parse)?
            .as_utf8_str();
        stat_ids.insert(id, c.inner_text(dom.parser()));
    }

    for (id, hero) in stat_ids {
        let stats_selector = format!(".stats-container.option-{id}");
        let stats = find2(dom, container, &stats_selector).ok_or(Error::Parse)?;
        let mut hero_stats = HashMap::new();
        for stat in find_all2(dom, stats, ".stat-item") {
            let name = find_inner_text2(dom, stat, ".name").ok_or(Error::Parse)?;
            let value = find_inner_text2(dom, stat, ".value").ok_or(Error::Parse)?;
            hero_stats.insert(name, value.parse().map_err(|()| Error::Parse)?);
        }
        heroes.insert(hero.to_string(), HeroStats { stats: hero_stats });
    }

    Ok(heroes)
}

fn ranks<'dom>(dom: &'dom VDom<'dom>) -> crate::Result<Vec<Rank>> {
    let mut ranks = Vec::new();
    for rank_wrapper in find_all(dom, ".Profile-playerSummary--rankWrapper") {
        let console = rank_wrapper.attributes().is_class_member("controller-view");
        for role_wrapper in find_all2(dom, rank_wrapper, ".Profile-playerSummary--roleWrapper") {
            let rank_url = find_attr2(dom, role_wrapper, ".Profile-playerSummary--rank", "src")
                .ok_or(Error::Parse)?;

            let split = url_file(&rank_url)?.split_once('-').ok_or(Error::Parse)?;
            let group = match split.0 {
                "BronzeTier" => Group::Bronze,
                "SilverTier" => Group::Silver,
                "GoldTier" => Group::Gold,
                "PlatinumTier" => Group::Platinum,
                "DiamondTier" => Group::Diamond,
                "MasterTier" => Group::Master,
                "GrandmasterTier" => Group::Grandmaster,
                _ => return Err(Error::Parse),
            };
            #[allow(clippy::string_slice)]
            let division: Division = split.1[..1].parse().map_err(|_| Error::Parse)?;

            let role_url = if console {
                find_attr2(dom, role_wrapper, "[xlink:href]", "xlink:href").ok_or(Error::Parse)?
            } else {
                find_attr2(dom, role_wrapper, "[src]", "src").ok_or(Error::Parse)?
            };

            let role = match url_file(&role_url)? {
                url if url.starts_with("tank") => Role::Tank,
                url if url.starts_with("offense") => Role::Damage,
                url if url.starts_with("support") => Role::Support,
                _ => return Err(Error::Parse),
            };

            hero_summary(dom, "quickPlay-view")?;
            hero_summary(dom, "competitive-view")?;

            ranks.push(Rank {
                group,
                division,
                role,
                console,
            });
        }
    }
    Ok(ranks)
}

fn endorsement<'dom>(dom: &'dom VDom<'dom>) -> crate::Result<Endorsement> {
    let endorsement_url =
        find_attr(dom, ".Profile-playerSummary--endorsement", "src").ok_or(crate::Error::Parse)?;

    #[allow(clippy::string_slice)]
    url_file(&endorsement_url)?[..1]
        .parse()
        .map_err(|_| Error::Parse)
}

fn portrait<'dom>(dom: &'dom VDom<'dom>) -> crate::Result<Url> {
    find_attr(dom, ".Profile-player--portrait", "src")
        .ok_or(Error::Parse)?
        .parse()
        .map_err(|_| Error::Parse)
}
