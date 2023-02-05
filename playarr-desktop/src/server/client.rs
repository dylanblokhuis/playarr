use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use ::serde::de::DeserializeOwned;
use tokio::runtime::Builder;
use tokio::sync::mpsc;

use crate::server::serde::Shows;

pub enum Fetch {
    Shows,
    Episodes(i64),
    Episode(i64),
}

fn get_url_for_fetch(fetch: &Fetch) -> String {
    match fetch {
        Fetch::Shows => "http://localhost:8000/shows".into(),
        Fetch::Episodes(id) => format!("http://localhost:8000/shows/{id}/episodes"),
        Fetch::Episode(id) => format!("http://localhost:8000/episode/{id}"),
    }
}

async fn handle_task(fetch: Fetch) -> (String, String) {
    let url: String = get_url_for_fetch(&fetch);

    let resp = reqwest::get(url.clone())
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    (url, resp)
}

#[derive(Clone)]
pub struct Client {
    spawn: mpsc::Sender<Fetch>,
    cache: Arc<RwLock<HashMap<String, String>>>,
}

impl Client {
    pub fn new() -> Client {
        let (send, mut recv) = mpsc::channel(16);
        let cache = Arc::new(RwLock::new(HashMap::<String, String>::new()));

        let rt = Builder::new_current_thread().enable_all().build().unwrap();

        let cache2 = cache.clone();
        std::thread::spawn(move || {
            rt.block_on(async move {
                while let Some(task) = recv.recv().await {
                    let result = tokio::spawn(handle_task(task));
                    let (key, result) = result.await.unwrap();
                    cache2.write().unwrap().insert(key, result);
                }
            });
        });

        Client { spawn: send, cache }
    }

    /**
     *  gets data from the cache or sends a request to the async runtime
     *  currently deserializes the data every single call, probably should just cache the structs in memory
     */
    fn fetch<T>(&self, fetch: Fetch) -> Option<T>
    where
        T: DeserializeOwned,
    {
        let cache = self.cache.read().unwrap();
        let url = get_url_for_fetch(&fetch);

        if let Some(json) = cache.get(&url) {
            let data: T = serde_json::from_str(json).unwrap();
            return Some(data);
        }

        match self.spawn.blocking_send(fetch) {
            Ok(()) => None,
            Err(_) => panic!("The async runtime has shut down."),
        }
    }

    pub fn get_all_series(&self) -> Option<Shows> {
        self.fetch::<Shows>(Fetch::Shows)
    }
}
