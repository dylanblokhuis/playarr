use ::serde::de::DeserializeOwned;

use crate::server::serde::Shows;

use super::{
    serde::{EpisodeDetail, Episodes},
    NetworkCache,
};

pub enum Fetch {
    Shows,
    Episodes(i64),
    Episode(i64),
}

fn get_url_for_fetch(fetch: &Fetch) -> String {
    match fetch {
        Fetch::Shows => "http://localhost:8000/shows".into(),
        Fetch::Episodes(id) => format!("http://localhost:8000/shows/{id}/episodes"),
        Fetch::Episode(id) => format!("http://localhost:8000/episodes/{id}"),
    }
}

#[derive(Clone)]
pub struct Client {
    cache: NetworkCache,
}

impl Client {
    pub fn new(cache: NetworkCache) -> Client {
        Client { cache }
    }

    /**
     *  gets data from the cache or sends a request to the async runtime
     *  currently deserializes the data every single call, probably should just cache the structs in memory
     */
    fn fetch<T>(&self, fetch: Fetch) -> Option<T>
    where
        T: DeserializeOwned,
    {
        let url = get_url_for_fetch(&fetch);

        if let Some(bytes) = self.cache.fetch(url) {
            let data: T = serde_json::from_slice(&bytes).unwrap();
            return Some(data);
        }

        None
    }

    pub fn get_all_series(&self) -> Option<Shows> {
        self.fetch::<Shows>(Fetch::Shows)
    }
    pub fn get_episodes(&self, id: i64) -> Option<Episodes> {
        self.fetch::<Episodes>(Fetch::Episodes(id))
    }
    pub fn get_episode(&self, id: i64) -> Option<EpisodeDetail> {
        self.fetch::<EpisodeDetail>(Fetch::Episode(id))
    }
}
