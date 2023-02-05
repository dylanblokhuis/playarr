use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use egui_extras::RetainedImage;
use tokio::runtime::Builder;
use tokio::sync::mpsc;

async fn handle_task(url: String) -> (String, Arc<RetainedImage>) {
    let resp = reqwest::get(url.clone())
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();

    (
        url.clone(),
        Arc::new(egui_extras::RetainedImage::from_image_bytes(url, &resp).unwrap()),
    )
}

#[derive(Clone)]
pub struct NetworkImageCache {
    spawn: mpsc::Sender<String>,
    cache: Arc<RwLock<HashMap<String, Arc<RetainedImage>>>>,
    is_loading: Arc<RwLock<HashMap<String, bool>>>,
}

impl NetworkImageCache {
    pub fn new() -> NetworkImageCache {
        let (send, mut recv) = mpsc::channel(16);
        let cache = Arc::new(RwLock::new(HashMap::<String, Arc<RetainedImage>>::new()));
        let is_loading = Arc::new(RwLock::new(HashMap::<String, bool>::new()));

        let rt = Builder::new_current_thread().enable_all().build().unwrap();

        let cache2 = cache.clone();
        let is_loading2 = is_loading.clone();

        std::thread::spawn(move || {
            rt.block_on(async move {
                while let Some(task) = recv.recv().await {
                    let cache_clone = cache.clone();
                    let is_loading_clone = is_loading.clone();
                    tokio::spawn(async move {
                        let (key, result) = handle_task(task).await;
                        cache_clone
                            .clone()
                            .write()
                            .unwrap()
                            .insert(key.clone(), result);
                        is_loading_clone.write().unwrap().insert(key, false);
                    });
                }
            });
        });

        NetworkImageCache {
            spawn: send,
            cache: cache2,
            is_loading: is_loading2,
        }
    }

    pub fn fetch_image(&self, url: String) -> Option<Arc<RetainedImage>> {
        let cache = self.cache.read().unwrap();

        if let Some(image) = cache.get(&url) {
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
