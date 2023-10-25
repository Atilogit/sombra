mod error;

use std::collections::HashMap;

use chrono::{serde::ts_seconds, DateTime, Utc};
use error::Result;

use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use serde_derive::{Deserialize, Serialize};
use sombra::{Asset, Battletag, Client, Endorsement, Id, PlayerProfile, Rank};
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FoundPlayer {
    pub battle_tag: Battletag,
    #[serde(with = "ts_seconds")]
    pub last_updated: DateTime<Utc>,
    pub is_public: bool,
    pub namecard: Option<Url>,
    pub portrait: Option<Url>,
    pub title: Option<HashMap<String, String>>,
}

async fn assets(State(client): State<Client>) -> Result<Json<HashMap<Id, Asset>>> {
    Ok(Json(assets_inner(client).await?))
}

async fn search(
    State(client): State<Client>,
    Path(name): Path<String>,
) -> Result<Json<Vec<FoundPlayer>>> {
    let assets = assets_inner(client.clone()).await?;
    let found = client.search(&name).await?;
    Ok(Json(
        found
            .into_iter()
            .map(|f| {
                let namecard = f.namecard.and_then(|id| assets[&id].icon.clone());
                let portrait = f.portrait.and_then(|id| assets[&id].icon.clone());
                let title = f.title.map(|id| assets[&id].name.clone());
                FoundPlayer {
                    battle_tag: f.battle_tag,
                    last_updated: f.last_updated,
                    is_public: f.is_public,
                    namecard,
                    portrait,
                    title,
                }
            })
            .collect(),
    ))
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerProfileReduced {
    pub battletag: Battletag,
    pub title: Option<String>,
    pub endorsement: Endorsement,
    pub portrait: Url,
    pub ranks: Vec<Rank>,
    pub private: bool,
    #[serde(with = "ts_seconds")]
    pub last_updated: DateTime<Utc>,
}
async fn profile_full(
    State(client): State<Client>,
    Path(btag): Path<Battletag>,
) -> Result<Json<PlayerProfile>> {
    Ok(Json(profile_inner(client, btag).await?))
}

async fn profile(
    State(client): State<Client>,
    Path(btag): Path<Battletag>,
) -> Result<Json<PlayerProfileReduced>> {
    let p = profile_inner(client, btag).await?;
    Ok(Json(PlayerProfileReduced {
        battletag: p.battletag,
        title: p.title,
        endorsement: p.endorsement,
        portrait: p.portrait,
        ranks: p.ranks,
        private: p.private,
        last_updated: p.last_updated,
    }))
}

async fn profile_inner(client: Client, btag: Battletag) -> Result<PlayerProfile> {
    Ok(client.profile(btag).await?)
}

async fn assets_inner(client: Client) -> Result<HashMap<Id, Asset>> {
    Ok(client.assets().await?)
}

#[shuttle_runtime::main]
#[allow(clippy::unused_async)]
async fn main() -> shuttle_axum::ShuttleAxum {
    let client = Client::new();
    let router = Router::new()
        .route("/search/:name", get(search))
        .route("/profile_full/:battletag", get(profile_full))
        .route("/profile/:battletag", get(profile))
        .route("/assets", get(assets))
        .with_state(client);

    Ok(router.into())
}
