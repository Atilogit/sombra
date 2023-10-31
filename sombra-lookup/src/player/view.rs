use std::collections::BTreeMap;

use leptos::{IntoView, *};
use sombra_client::{Division, Group, Hero, Rank, Role, Stat};

use crate::player::Stats;

use super::Player;

impl Player {
    #[allow(clippy::too_many_lines)]
    pub fn view(&self) -> impl IntoView {
        let mut hero_stats = self
            .heroes
            .iter()
            .filter_map(|hero| Some((self.hero_stats(&hero.name)?, hero)))
            .collect::<Vec<_>>();
        hero_stats.sort_by_key(|(stats, _)| stats["Time Played"].as_duration());
        hero_stats.reverse();
        let hero_bars = hero_stats
            .iter()
            .enumerate()
            .map(|(z, (stats, hero))| {
                let time = stats["Time Played"].as_duration().unwrap();
                view! { <HeroBar stats=&stats hero=hero proportion=time.as_secs_f64() z={hero_stats.len() - z} /> }
            })
            .collect_view();

        view! {
            <div
                class="cursor-default overflow-hidden whitespace-nowrap rounded-2xl font-['Inter'] drop-shadow-xl h-fit">
                <div class="relative">
                    <div class="from-base-300 absolute h-full w-full bg-gradient-to-l opacity-100"></div>
                    <div class="bg-base-300 absolute h-full w-full opacity-75"></div>
                    <div class="absolute grid h-full w-full grid-cols-2 grid-rows-2 items-center px-6 py-4 text-white">
                        <div class="self-start">
                            <div class="text-xl font-bold">{self.btag.to_string()}</div>
                            {self.title()}
                        </div>

                        <div class="grid grid-cols-3">
                            { self.rank(Role::Tank).map(|rank| view!{ <Rank rank=&rank />} ) }
                            { self.rank(Role::Damage).map(|rank| view!{ <Rank rank=&rank />} ) }
                            { self.rank(Role::Support).map(|rank| view!{ <Rank rank=&rank />} ) }
                        </div>

                        <div class="grid grid-cols-3">
                            <div class="font-bold">
                                <div>Time Played</div>
                                <div>Record</div>
                            </div>
                            <div class="col-span-2 font-semibold">
                                <div>7h</div>
                                <div>
                                    <span class="text-green-500">25</span> - 0 - <span class="text-red-500">16</span>
                                </div>
                            </div>
                        </div>

                        <div class="grid grid-cols-3 font-semibold">
                            { self.role_stats(Role::Tank).map(|stats| view!{ <TopStat stats=&stats />} ) }
                            { self.role_stats(Role::Damage).map(|stats| view!{ <TopStat stats=&stats />} ) }
                            { self.role_stats(Role::Support).map(|stats| view!{ <TopStat stats=&stats />} ) }
                        </div>
                    </div>
                    <img src={self.namecard_url()} />
                </div>
                {(!hero_stats.is_empty()).then(|| view! {
                    <div class="flex h-20 w-full text-black">
                        {hero_bars}
                    </div>
                })}

            </div>
        }
    }
}

#[component]
fn HeroBar<'st>(
    stats: &'st BTreeMap<String, Stat>,
    hero: &'st Hero,
    proportion: f64,
    z: usize,
) -> impl IntoView {
    view! {
        <div class="group flex w-0 flex-auto items-center justify-center bg-pink-400 shadow transition-all hover:w-full"
            style={format!("flex-grow: {proportion}; z-index: {z}; box-shadow: 0px 4px 4px 4px #0004;")}>
            <div class="flex h-full flex-wrap items-center">
                <div class="ml-px h-full group-hover:hidden"></div>
                <img src=hero.portrait.to_string() class="h-14 w-14 rounded-full group-hover:ml-4" />
            </div>
            <div class="flex w-0 justify-around group-hover:w-full">
                {
                    stats.iter().map(|stat| {
                        view! { <HeroBarStat name=stat.0 stat=stat.1 /> }
                    }).collect_view()
                }
            </div>
        </div>
    }
}

#[component]
fn HeroBarStat<'st>(name: &'st String, stat: &'st Stat) -> impl IntoView {
    view! {
        <div class="flex-grow-0 overflow-hidden opacity-20 transition-all group-hover:opacity-100">
            <div>{name}</div>
            <div class="font-semibold">{stat.to_string()}</div>
        </div>
    }
}

#[component]
fn TopStat<'st>(stats: &'st Stats) -> impl IntoView {
    view! {
        <div class="text-center">
            <div>{format!("{:?}", stats.time)}</div>
            <div><span class="text-green-500">{stats.win}</span> - {stats.draw} - <span class="text-red-500">{stats.loss}</span></div>
        </div>
    }
}

#[component]
fn rank<'ra>(rank: &'ra Rank) -> impl IntoView {
    view! {
        <div class="text-center">
            <div class="inline-block w-8 text-center">
                <img src=role_icon_url(rank.role) class="inline-block h-8" />
            </div>
            <div class="inline-block w-14 text-center">
                <img src=rank_icon_url(rank.group, rank.division) class="inline-block h-12" />
            </div>
        </div>
    }
}

const fn role_icon_url(role: Role) -> &'static str {
    match role {
        Role::Damage => "https://static.playoverwatch.com/img/pages/career/icons/role/offense-ab1756f419.svg#icon",
        Role::Tank => "https://static.playoverwatch.com/img/pages/career/icons/role/tank-f64702b684.svg#icon",
        Role::Support => "https://static.playoverwatch.com/img/pages/career/icons/role/support-0258e13d85.svg#icon",
    }
}

fn rank_icon_url(group: Group, division: Division) -> String {
    let group = format!("{group:?}",).to_lowercase();
    format!(
        "https://www.overbuff.com/_next/image?url=%2FskillDivisions%2F{group}-{division}.png&w=750&q=75"
    )
}
