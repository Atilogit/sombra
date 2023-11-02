#![allow(clippy::unused_async)]
#![allow(clippy::useless_let_if_seq)]

mod error;

use error::Result;

use std::{collections::HashMap, sync::Arc};

use poem::{endpoint::StaticFilesEndpoint, middleware, EndpointExt, Route};
use poem_openapi::{param::Query, payload::Json, ContactObject, OpenApi, OpenApiService, Tags};
use shuttle_poem::ShuttlePoem;
use sombra::{
    Asset, Battletag, CachedClient, FoundPlayer, Hero, Id, Overbuff, PlayerProfile,
    PlayerProfileReduced,
};

struct Api {
    client: Arc<CachedClient>,
}

#[derive(Tags)]
enum ApiTags {
    V1,
}

#[OpenApi(prefix_path = "/v1", tag = "ApiTags::V1")]
impl Api {
    async fn new() -> Self {
        #[cfg(not(debug_assertions))]
        let client = Arc::new(CachedClient::new_default().await.unwrap());
        #[cfg(debug_assertions)]
        let client = Arc::new(CachedClient::new_max().await.unwrap());
        Self { client }
    }

    #[oai(path = "/search", method = "get")]
    async fn search(&self, Query(name): Query<String>) -> Result<Json<Vec<FoundPlayer>>> {
        Ok(Json(self.client.search(&name).await?))
    }

    #[oai(path = "/profile", method = "get")]
    async fn profile(
        &self,
        Query(name): Query<String>,
        Query(number): Query<u64>,
    ) -> Result<Json<PlayerProfileReduced>> {
        let btag = Battletag::new(name, number);
        Ok(Json(self.client.profile(&btag).await?))
    }

    #[oai(path = "/profile_full", method = "get")]
    async fn profile_full(
        &self,
        Query(name): Query<String>,
        Query(number): Query<u64>,
    ) -> Result<Json<PlayerProfile>> {
        let btag = Battletag::new(name, number);
        Ok(Json(self.client.profile_full(&btag).await?))
    }

    #[oai(path = "/overbuff", method = "get")]
    async fn overbuff(
        &self,
        Query(name): Query<String>,
        Query(number): Query<u64>,
    ) -> Result<Json<Overbuff>> {
        let btag = Battletag::new(name, number);
        Ok(Json(self.client.overbuff(&btag).await?))
    }

    #[oai(path = "/assets", method = "get")]
    async fn assets(&self) -> Json<&HashMap<Id, Asset>> {
        Json(self.client.assets())
    }

    #[oai(path = "/heroes", method = "get")]
    async fn heroes(&self) -> Json<&[Hero]> {
        Json(self.client.heroes())
    }
}

#[shuttle_runtime::main]
async fn poem() -> ShuttlePoem<impl poem::Endpoint> {
    let api_service = OpenApiService::new(Api::new().await, "Sombra", env!("CARGO_PKG_VERSION"))
        .contact(
            ContactObject::new()
                .url("https://atilo.sh")
                .name("by atilo"),
        )
        .url_prefix("/api")
        .external_document("https://github.com/Atilogit/sombra")
        .license("MIT License");
    let ui = api_service.swagger_ui();
    let spec_json = api_service.spec_endpoint();
    let spec_yaml = api_service.spec_endpoint_yaml();
    let app = Route::new()
        .nest("/api", api_service)
        .nest("/docs", ui)
        .nest("/spec.json", spec_json)
        .nest("/spec.yaml", spec_yaml)
        .nest(
            "/",
            StaticFilesEndpoint::new("sombra-lookup/dist/")
                .fallback_to_index()
                .index_file("index.html"),
        )
        .with(middleware::Compression::new())
        .with(middleware::Tracing);

    Ok(app.into())
}
