use std::sync::{Arc, RwLock};

use ::serde::de::DeserializeOwned;

use crate::{server::serde::Shows, ui::Config};

use super::{
    network_cache::FetchResult,
    serde::{EpisodeDetail, Episodes},
    NetworkCache,
};

pub enum Fetch {
    Shows,
    Episodes(i64),
    Episode(i64),
}

#[derive(Clone)]
pub struct Client {
    config: Arc<RwLock<Config>>,
    cache: NetworkCache,
}

impl Client {
    pub fn new(config: Arc<RwLock<Config>>, cache: NetworkCache) -> Client {
        Client { config, cache }
    }

    fn get_url_for_fetch(&self, fetch: &Fetch) -> String {
        let server_address = self.config.read().unwrap().server_address.clone();
        match fetch {
            Fetch::Shows => format!("{server_address}/shows"),
            Fetch::Episodes(id) => format!("{server_address}/shows/{id}/episodes"),
            Fetch::Episode(id) => format!("{server_address}/episodes/{id}"),
        }
    }
    /**
     *  gets data from the cache or sends a request to the async runtime
     *  currently deserializes the data every single call, probably should just cache the structs in memory
     */
    fn fetch<T>(&self, fetch: Fetch) -> FetchResult<T>
    where
        T: DeserializeOwned,
    {
        let url = self.get_url_for_fetch(&fetch);

        match self.cache.fetch(url) {
            FetchResult::Ok(bytes) => {
                let data: Result<T, serde_json::Error> = serde_json::from_slice(&bytes);
                match data {
                    Ok(data) => FetchResult::Ok(data),
                    Err(err) => FetchResult::Error(err.to_string()),
                }
            }
            FetchResult::Error(err) => FetchResult::Error(err),
            FetchResult::Loading => FetchResult::Loading,
        }
    }

    pub fn get_all_series(&self) -> FetchResult<Shows> {
        self.fetch::<Shows>(Fetch::Shows)
    }
    pub fn get_episodes(&self, id: i64) -> FetchResult<Episodes> {
        self.fetch::<Episodes>(Fetch::Episodes(id))
    }
    pub fn get_episode(&self, id: i64) -> FetchResult<EpisodeDetail> {
        self.fetch::<EpisodeDetail>(Fetch::Episode(id))
    }
}
