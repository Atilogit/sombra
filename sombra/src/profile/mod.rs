mod types;
mod util;

use self::util::{find, find2, find_all, find_all2, find_attr2, find_inner_text2};
use crate::{Battletag, Client, Error};
use chrono::{DateTime, TimeZone, Utc};
use std::collections::HashMap;
use tl::{ParserOptions, VDom};
use tracing::instrument;
pub use types::*;
use url::Url;
use util::{find_attr, find_inner_text, url_file};

impl Client {
    #[instrument(skip(self))]
    pub async fn profile(&self, btag: Battletag) -> crate::Result<PlayerProfile> {
        let url = "https://overwatch.blizzard.com/en-us/career/";
        let html = self.get(&format!("{url}{btag:#}/")).await?;
        let dom = tl::parse(&html, ParserOptions::new())?;

        let public = find(&dom, ".Profile-player--privateText").is_none();
        let (quickplay_console, competitive_console, quickplay_pc, competitive_pc);
        if public {
            quickplay_console = hero_stats(&dom, true, true)?;
            competitive_console = hero_stats(&dom, false, true)?;
            quickplay_pc = hero_stats(&dom, true, false)?;
            competitive_pc = hero_stats(&dom, false, false)?;
        } else {
            quickplay_console = HashMap::new();
            competitive_console = HashMap::new();
            quickplay_pc = HashMap::new();
            competitive_pc = HashMap::new();
        }

        Ok(PlayerProfile {
            battletag: btag,
            title: find_inner_text(&dom, ".Profile-player--title"),
            endorsement: endorsement(&dom)?,
            portrait: portrait(&dom)?,
            ranks: ranks(&dom)?,
            private: !public,
            last_updated: last_update(&dom)?,
            quickplay_console,
            competitive_console,
            quickplay_pc,
            competitive_pc,
        })
    }
}

#[instrument(skip_all)]
fn hero_stats<'dom>(
    dom: &'dom VDom<'dom>,
    qp: bool,
    console: bool,
) -> crate::Result<HashMap<String, HeroStats>> {
    let mut heroes = HashMap::new();

    let view_selector = if console {
        ".Profile-view.controller-view"
    } else {
        ".Profile-view.mouseKeyboard-view"
    };
    if let Some(view) = find(dom, view_selector) {
        let container_selector = if qp {
            ".stats.quickPlay-view"
        } else {
            ".stats.competitive-view"
        };
        if let Some(container) = find2(dom, view, container_selector) {
            if let Some(select) = find2(dom, container, ".Profile-dropdown") {
                let mut stat_ids = HashMap::new();
                for c in find_all2(dom, select, "option") {
                    let id = c
                        .attributes()
                        .get("value")
                        .flatten()
                        .ok_or_else(Error::parse)?
                        .as_utf8_str();
                    stat_ids.insert(id, c.inner_text(dom.parser()));
                }

                for (id, hero) in stat_ids {
                    let stats_selector = format!(".stats-container.option-{id}");
                    let stats = find2(dom, container, &stats_selector).ok_or_else(Error::parse)?;
                    let mut hero_stats = HashMap::new();
                    for stat in find_all2(dom, stats, ".stat-item") {
                        let name = find_inner_text2(dom, stat, ".name").ok_or_else(Error::parse)?;
                        let value =
                            find_inner_text2(dom, stat, ".value").ok_or_else(Error::parse)?;
                        hero_stats.insert(name, value.parse().map_err(|()| Error::parse())?);
                    }
                    heroes.insert(hero.to_string(), HeroStats { stats: hero_stats });
                }
            }
        }
    }
    Ok(heroes)
}

#[instrument(skip_all)]
fn ranks<'dom>(dom: &'dom VDom<'dom>) -> crate::Result<Vec<Rank>> {
    let mut ranks = Vec::new();
    for rank_wrapper in find_all(dom, ".Profile-playerSummary--rankWrapper") {
        let console = rank_wrapper.attributes().is_class_member("controller-view");
        for role_wrapper in find_all2(dom, rank_wrapper, ".Profile-playerSummary--roleWrapper") {
            let rank_url = find_attr2(dom, role_wrapper, ".Profile-playerSummary--rank", "src")
                .ok_or_else(Error::parse)?;

            let split = url_file(&rank_url)?
                .split_once('-')
                .ok_or_else(Error::parse)?;
            let group = match split.0 {
                "BronzeTier" => Group::Bronze,
                "SilverTier" => Group::Silver,
                "GoldTier" => Group::Gold,
                "PlatinumTier" => Group::Platinum,
                "DiamondTier" => Group::Diamond,
                "MasterTier" => Group::Master,
                "GrandmasterTier" => Group::Grandmaster,
                _ => return Err(Error::parse()),
            };
            #[allow(clippy::string_slice)]
            let division: Division = split.1[..1].parse().map_err(|_| Error::parse())?;

            let role_url = if console {
                find_attr2(dom, role_wrapper, "[xlink:href]", "xlink:href")
                    .ok_or_else(Error::parse)?
            } else {
                find_attr2(dom, role_wrapper, "[src]", "src").ok_or_else(Error::parse)?
            };

            let role = match url_file(&role_url)? {
                url if url.starts_with("tank") => Role::Tank,
                url if url.starts_with("offense") => Role::Damage,
                url if url.starts_with("support") => Role::Support,
                _ => return Err(Error::parse()),
            };

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

#[instrument(skip_all)]
fn endorsement<'dom>(dom: &'dom VDom<'dom>) -> crate::Result<Endorsement> {
    let endorsement_url =
        find_attr(dom, ".Profile-playerSummary--endorsement", "src").ok_or_else(Error::parse)?;

    #[allow(clippy::string_slice)]
    url_file(&endorsement_url)?[..1]
        .parse()
        .map_err(|_| Error::parse())
}

#[instrument(skip_all)]
fn portrait<'dom>(dom: &'dom VDom<'dom>) -> crate::Result<Url> {
    find_attr(dom, ".Profile-player--portrait", "src")
        .ok_or_else(Error::parse)?
        .parse()
        .map_err(|_| Error::parse())
}

#[instrument(skip_all)]
fn last_update<'dom>(dom: &'dom VDom<'dom>) -> crate::Result<DateTime<Utc>> {
    let ts_str = find_attr(dom, ".Profile-masthead", "data-lastUpdate").ok_or_else(Error::parse)?;
    let ts: i64 = ts_str.parse().map_err(|_| Error::parse())?;
    Utc.timestamp_opt(ts, 0).single().ok_or_else(Error::parse)
}
