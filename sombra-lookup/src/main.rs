#![allow(clippy::wildcard_imports)]
#![allow(clippy::default_numeric_fallback)]
#![allow(clippy::same_name_method)]
#![allow(clippy::empty_structs_with_brackets)]

mod player;

use std::str::FromStr;

use leptos::*;
use leptos_use::storage::use_storage;
use sombra_client::{Battletag, Client};

use crate::player::Player;

fn main() {
    leptos::mount_to_body(|| view! { <App/> });
}

#[component]
fn App() -> impl IntoView {
    let btag_regex = regex::Regex::new(r"[^\pZ\pC#]*#[0-9]+").unwrap();

    #[cfg(not(debug_assertions))]
    let (btag_input, set_btag_input) = create_signal(String::new());
    #[cfg(debug_assertions)]
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
                let heroes = client.heroes().await.ok().unwrap_or(Vec::new());
                let player = Player {
                    btag,
                    profile,
                    overbuff,
                    found,
                    heroes,
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
        <div class="container mx-auto mt-32 mb-16 min-h-screen">
            <textarea class="textarea textarea-bordered w-full mb-8" on:input=on_btag_input prop:value=btag_input rows=10 />

            <div class="grid gap-4 grid-cols-2">
                <For
                    each=move || players.get()
                    key=|p| p.btag.clone()
                    let:p
                >
                    {p.view()}
                </For>
            </div>
        </div>
        <footer class="footer p-10 bg-neutral text-neutral-content">
            <nav>
            <header class="footer-title">Social</header>
            <div class="grid grid-flow-col gap-4">
            <a><svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" class="fill-current"><path d="M24 4.557c-.883.392-1.832.656-2.828.775 1.017-.609 1.798-1.574 2.165-2.724-.951.564-2.005.974-3.127 1.195-.897-.957-2.178-1.555-3.594-1.555-3.179 0-5.515 2.966-4.797 6.045-4.091-.205-7.719-2.165-10.148-5.144-1.29 2.213-.669 5.108 1.523 6.574-.806-.026-1.566-.247-2.229-.616-.054 2.281 1.581 4.415 3.949 4.89-.693.188-1.452.232-2.224.084.626 1.956 2.444 3.379 4.6 3.419-2.07 1.623-4.678 2.348-7.29 2.04 2.179 1.397 4.768 2.212 7.548 2.212 9.142 0 14.307-7.721 13.995-14.646.962-.695 1.797-1.562 2.457-2.549z"></path></svg></a>
            <a><svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" class="fill-current"><path d="M19.615 3.184c-3.604-.246-11.631-.245-15.23 0-3.897.266-4.356 2.62-4.385 8.816.029 6.185.484 8.549 4.385 8.816 3.6.245 11.626.246 15.23 0 3.897-.266 4.356-2.62 4.385-8.816-.029-6.185-.484-8.549-4.385-8.816zm-10.615 12.816v-8l8 3.993-8 4.007z"></path></svg></a>
            <a><svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" class="fill-current"><path d="M9 8h-3v4h3v12h5v-12h3.642l.358-4h-4v-1.667c0-.955.192-1.333 1.115-1.333h2.885v-5h-3.808c-3.596 0-5.192 1.583-5.192 4.615v3.385z"></path></svg></a>
            </div>
            </nav>
        </footer>
    }
}
