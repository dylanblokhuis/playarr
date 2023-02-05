use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use bytes::Bytes;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;

async fn handle_task(url: String) -> (String, Bytes) {
    let now = std::time::Instant::now();
    let resp = reqwest::get(url.clone())
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();
    println!("{} - {}ms", url, now.elapsed().as_millis());

    (url.clone(), resp)
}

#[derive(Clone)]
pub struct NetworkCache {
    spawn: mpsc::Sender<String>,
    cache: Arc<RwLock<HashMap<String, Bytes>>>,
    is_loading: Arc<RwLock<HashMap<String, bool>>>,
}

impl NetworkCache {
    pub fn new(rt: Runtime, ctx: egui::Context) -> NetworkCache {
        let (send, mut recv) = mpsc::channel(16);
        let cache = Arc::new(RwLock::new(HashMap::<String, Bytes>::new()));
        let is_loading = Arc::new(RwLock::new(HashMap::<String, bool>::new()));

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
                        cache_clone
                            .clone()
                            .write()
                            .unwrap()
                            .insert(key.clone(), result);
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

    pub fn fetch(&self, url: String) -> Option<Bytes> {
        if let Some(image) = self.cache.read().unwrap().get(&url) {
            return Some(image.clone());
        }

        if let Some(is_loading) = self.is_loading.read().unwrap().get(&url) {
            if *is_loading {
                return None;
            }
        }

        self.is_loading.write().unwrap().insert(url.clone(), true);

        match self.spawn.blocking_send(url) {
            Ok(()) => None,
            Err(_) => panic!("The async runtime has shut down."),
        }
    }
}
