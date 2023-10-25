mod error;

use std::sync::Arc;

use error::Result;

use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use sombra::{Battletag, CachedClient, FoundPlayer, Overbuff, PlayerProfile, PlayerProfileReduced};

async fn search(
    State(client): State<Arc<CachedClient>>,
    Path(name): Path<String>,
) -> Result<Json<Vec<FoundPlayer>>> {
    Ok(Json(client.search(&name).await?))
}

async fn profile_full(
    State(client): State<Arc<CachedClient>>,
    Path(btag): Path<Battletag>,
) -> Result<Json<PlayerProfile>> {
    Ok(Json(client.profile_full(&btag).await?))
}

async fn profile(
    State(client): State<Arc<CachedClient>>,
    Path(btag): Path<Battletag>,
) -> Result<Json<PlayerProfileReduced>> {
    Ok(Json(client.profile(&btag).await?))
}

async fn overbuff(
    State(client): State<Arc<CachedClient>>,
    Path(btag): Path<Battletag>,
) -> Result<Json<Overbuff>> {
    Ok(Json(client.overbuff(&btag).await?))
}

#[shuttle_runtime::main]
#[allow(clippy::unused_async)]
async fn main() -> shuttle_axum::ShuttleAxum {
    let client = Arc::new(CachedClient::new().await.unwrap());
    let router = Router::new()
        .route("/search/:name", get(search))
        .route("/profile_full/:battletag", get(profile_full))
        .route("/profile/:battletag", get(profile))
        .route("/overbuff/:battletag", get(overbuff))
        .with_state(client);

    Ok(router.into())
}
