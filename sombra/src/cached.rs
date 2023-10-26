use cached::{Cached, TimedCache};
use parking_lot::Mutex;
use sombra_types::{Battletag, FoundPlayer, Overbuff, PlayerProfile, PlayerProfileReduced};

use crate::Client;

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
