#![allow(clippy::wildcard_imports)]
#![allow(clippy::default_numeric_fallback)]
#![allow(clippy::same_name_method)]
#![allow(clippy::empty_structs_with_brackets)]

use std::str::FromStr;

use leptos::*;
use leptos_use::storage::use_storage;
use serde_derive::{Deserialize, Serialize};
use sombra_client::{
    Battletag, Client, Division, FoundPlayer, Group, Overbuff, PlayerProfile, Rank, Role,
};

fn main() {
    leptos::mount_to_body(|| view! { <App/> });
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Player {
    btag: Battletag,
    profile: Option<PlayerProfile>,
    overbuff: Option<Overbuff>,
    found: FoundPlayer,
}

impl Player {
    fn namecard_url(&self) -> String {
        self.found
            .namecard
            .as_ref()
            .map_or(String::new(), std::string::ToString::to_string)
    }

    fn title(&self) -> String {
        self.found
            .title
            .as_ref()
            .map_or(String::new(), |t| t["en_US"].clone())
    }

    fn ranks(&self) -> Vec<Rank> {
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
}

#[component]
fn App() -> impl IntoView {
    let btag_regex = regex::Regex::new(r"[^\pZ\pC#]*#[0-9]+").unwrap();

    let (btag_input, set_btag_input, _) = use_storage("btag_input", String::new());
    let (btags, set_btags) = create_signal(Vec::<Battletag>::new());
    let (players, set_players) = create_signal(Vec::<Player>::new());

    let load_player = create_action(move |btag: &Battletag| {
        let btag = btag.clone();
        async move {
            // let client = Client::new("https://sombra.shuttleapp.rs/");
            let client = Client::new("http://127.0.0.1:8000");
            let found = client
                .search(&btag.name)
                .await
                .ok()
                .and_then(|v| v.into_iter().find(|p| p.battle_tag == btag));
            if let Some(found) = found {
                let profile = client.profile_full(&btag).await.ok();
                let overbuff = if profile.is_none()
                    || !found.is_public
                    || profile.as_ref().is_some_and(|p| p.ranks.is_empty())
                {
                    client.overbuff(&btag).await.ok()
                } else {
                    None
                };
                let player = Player {
                    btag,
                    profile,
                    overbuff,
                    found,
                };
                set_players.update(|players| players.push(player));
            }
        }
    });

    let update_btag_input = move || {
        for btag in btag_regex.find_iter(&btag_input.get()) {
            let btag = Battletag::from_str(btag.as_str()).unwrap();
            let exists = btags.with(|v| v.iter().any(|b| b == &btag));
            if !exists {
                set_btags.update(|v| v.push(btag.clone()));
                load_player.dispatch(btag);
            }
        }
    };
    update_btag_input();

    let on_btag_input = move |e| {
        set_btag_input.set(event_target_value(&e));
        update_btag_input();
    };

    view! {
        <div class="container mx-auto">
            <textarea class="textarea textarea-bordered w-full" on:input=on_btag_input prop:value=btag_input rows=10 />

            <div class="grid gap-4 grid-cols-2">
                <For
                    each=move || players.get()
                    key=|p| p.btag.clone()
                    let:p
                >
                    <PlayerView player=&p />
                </For>
            </div>
        </div>
    }
}

#[component]
fn PlayerView<'pl>(player: &'pl Player) -> impl IntoView {
    view! {
        <div class="card bg-base-100 shadow-xl image-full">
            <figure><img src=player.namecard_url() /></figure>
            <div class="card-body">
                <div class="flex justify-between">
                    <div>
                        <h2 class="text-xl font-bold">{ player.btag.to_string() }</h2>
                        <h3 class="text-large">{ player.title() }</h3>
                    </div>
                    <div class="flex flex-row-reverse flex-wrap">
                        {
                            player.ranks().into_iter().map(|r| {
                                view! {
                                    <div class="flex-none">
                                        <div class="w-8 ml-5 inline-block drop-shadow-lg text-center">
                                            <img src=role_icon_url(r.role) class="h-8 inline-block" />
                                        </div>
                                        <div class="w-14 inline-block drop-shadow-lg text-center">
                                            <img src=rank_icon_url(r.group, r.division) class="h-12 inline-block" />
                                        </div>
                                    </div>
                                }
                            }).collect_view()
                        }
                    </div>
                </div>
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
