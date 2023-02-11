use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use bytes::Bytes;
use reqwest::Response;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;

#[derive(Clone)]
pub enum FetchResult<T> {
    Loading,
    Error(String),
    Ok(T),
}

async fn handle_task(url: String) -> (String, Result<Response, reqwest::Error>) {
    let now = std::time::Instant::now();
    let res = reqwest::get(url.clone()).await;
    println!("{} - {}ms", url, now.elapsed().as_millis());
    if res.is_err() {
        return (url.clone(), res);
    }
    (url.clone(), Ok(res.unwrap()))
}

#[derive(Clone)]
pub struct NetworkCache {
    spawn: mpsc::Sender<String>,
    cache: Arc<RwLock<HashMap<String, FetchResult<Bytes>>>>,
    is_loading: Arc<RwLock<HashMap<String, bool>>>,
}

impl NetworkCache {
    pub fn new(rt: Runtime, ctx: egui::Context) -> NetworkCache {
        let (send, mut recv) = mpsc::channel(16);
        let cache = Arc::new(RwLock::new(HashMap::new()));
        let is_loading = Arc::new(RwLock::new(HashMap::new()));

        let cache2 = cache.clone();
        let is_loading2 = is_loading.clone();

        std::thread::spawn(move || {
            rt.block_on(async move {
                while let Some(task) = recv.recv().await {
                    let cache_clone = cache.clone();
                    let is_loading_clone = is_loading.clone();
                    let ctx_clone = ctx.clone();
                    tokio::spawn(async move {
                        let (key, result) = handle_task(task).await;

                        // response is not ok
                        if result.is_err() {
                            cache_clone.clone().write().unwrap().insert(
                                key.clone(),
                                FetchResult::Error(result.unwrap_err().to_string()),
                            );
                            is_loading_clone.write().unwrap().insert(key, false);
                            return;
                        }

                        // response is ok, but body is not ok
                        let body_result = result.unwrap().bytes().await;
                        if body_result.is_err() {
                            cache_clone.clone().write().unwrap().insert(
                                key.clone(),
                                FetchResult::Error(body_result.unwrap_err().to_string()),
                            );
                            is_loading_clone.write().unwrap().insert(key, false);
                            return;
                        }

                        cache_clone
                            .clone()
                            .write()
                            .unwrap()
                            .insert(key.clone(), FetchResult::Ok(body_result.unwrap()));
                        is_loading_clone.write().unwrap().insert(key, false);

                        ctx_clone.request_repaint();
                    });
                }
            });
        });

        NetworkCache {
            spawn: send,
            cache: cache2,
            is_loading: is_loading2,
        }
    }

    pub fn fetch(&self, url: String) -> FetchResult<Bytes> {
        if let Some(image) = self.cache.read().unwrap().get(&url) {
            return image.clone();
        }

        if let Some(is_loading) = self.is_loading.read().unwrap().get(&url) {
            if *is_loading {
                return FetchResult::Loading;
            }
        }

        self.is_loading.write().unwrap().insert(url.clone(), true);

        match self.spawn.blocking_send(url) {
            Ok(()) => FetchResult::Loading,
            Err(_) => panic!("The async runtime has shut down."),
        }
    }
}
