use cached::{Cached, TimedCache};
use chrono::{serde::ts_seconds, DateTime, Utc};
use parking_lot::Mutex;
use serde_derive::{Deserialize, Serialize};
use url::Url;

use crate::{Battletag, Client, Endorsement, FoundPlayer, Overbuff, PlayerProfile, Rank};

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

#[derive(Debug)]
pub struct CachedClient {
    client: Client,
    profile_cache: Mutex<TimedCache<Battletag, PlayerProfile>>,
    overbuff_cache: Mutex<TimedCache<Battletag, Overbuff>>,
    search_cache: Mutex<TimedCache<String, Vec<FoundPlayer>>>,
}

impl CachedClient {
    pub async fn new() -> crate::Result<Self> {
        Ok(Self {
            client: Client::new().await?,
            profile_cache: Mutex::new(TimedCache::with_lifespan(60 * 20)),
            overbuff_cache: Mutex::new(TimedCache::with_lifespan(60 * 20)),
            search_cache: Mutex::new(TimedCache::with_lifespan(60 * 20)),
        })
    }

    pub async fn profile_full(&self, btag: &Battletag) -> crate::Result<PlayerProfile> {
        {
            let mut cache = self.profile_cache.lock();
            if let Some(profile) = cache.cache_get(btag) {
                return Ok(profile.clone());
            }
        }
        let profile = self.client.profile(btag).await?;
        self.profile_cache
            .lock()
            .cache_set(btag.clone(), profile.clone());
        Ok(profile)
    }

    pub async fn profile(&self, btag: &Battletag) -> crate::Result<PlayerProfileReduced> {
        {
            let mut cache = self.profile_cache.lock();
            if let Some(profile) = cache.cache_get(btag) {
                return Ok(profile.into());
            }
        }
        let profile = self.client.profile(btag).await?;
        let reduced = (&profile).into();
        self.profile_cache.lock().cache_set(btag.clone(), profile);
        Ok(reduced)
    }

    #[allow(clippy::unit_arg)]
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::let_unit_value)]
    pub async fn overbuff(&self, btag: &Battletag) -> crate::Result<Overbuff> {
        {
            let mut cache = self.overbuff_cache.lock();
            if let Some(profile) = cache.cache_get(btag) {
                return Ok(profile.clone());
            }
        }
        let overbuff = self.client.overbuff(btag).await?;
        self.overbuff_cache
            .lock()
            .cache_set(btag.clone(), overbuff.clone());
        Ok(overbuff)
    }

    pub async fn search(&self, name: &str) -> crate::Result<Vec<FoundPlayer>> {
        {
            let mut cache = self.search_cache.lock();
            if let Some(profile) = cache.cache_get(name) {
                return Ok(profile.clone());
            }
        }
        let search = self.client.search(name).await?;
        self.search_cache
            .lock()
            .cache_set(name.to_owned(), search.clone());
        Ok(search)
    }
}

impl From<&PlayerProfile> for PlayerProfileReduced {
    fn from(value: &PlayerProfile) -> Self {
        Self {
            battletag: value.battletag.clone(),
            title: value.title.clone(),
            endorsement: value.endorsement,
            portrait: value.portrait.clone(),
            ranks: value.ranks.clone(),
            private: value.private,
            last_updated: value.last_updated,
        }
    }
}
