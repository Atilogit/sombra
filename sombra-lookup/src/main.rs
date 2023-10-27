#![allow(clippy::wildcard_imports)]
#![allow(clippy::default_numeric_fallback)]
#![allow(clippy::same_name_method)]
#![allow(clippy::empty_structs_with_brackets)]

use std::str::FromStr;

use leptos::*;
use leptos_use::storage::use_storage;
use serde_derive::{Deserialize, Serialize};
use sombra_client::{Battletag, Client, FoundPlayer, Overbuff, PlayerProfile, Rank};

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
        self.overbuff
            .as_ref()
            .map(|p| &p.ranks)
            .or_else(|| self.profile.as_ref().map(|p| &p.ranks))
            .cloned()
            .unwrap_or(Vec::new())
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
                                    <div class="badge badge-accent ml-3 flex-none">
                                        { format!("{:?} {:?} {}", r.role, r.group, r.division) }
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
