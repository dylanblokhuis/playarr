use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use egui_extras::RetainedImage;

use super::network_cache::FetchResult;
use super::NetworkCache;

#[derive(Clone)]
pub struct NetworkImageCache {
    cache: NetworkCache,
    is_on_gpu: Arc<RwLock<HashMap<String, Arc<RetainedImage>>>>,
    is_uploading: Arc<RwLock<HashMap<String, bool>>>,
    ctx: egui::Context,
}

impl NetworkImageCache {
    pub fn new(cache: NetworkCache, ctx: egui::Context) -> NetworkImageCache {
        NetworkImageCache {
            cache,
            is_uploading: Arc::new(RwLock::new(HashMap::new())),
            is_on_gpu: Arc::new(RwLock::new(HashMap::new())),
            ctx,
        }
    }

    pub fn fetch_image(&self, url: String) -> Option<Arc<RetainedImage>> {
        if let Some(image) = self.is_on_gpu.read().unwrap().get(&url) {
            return Some(image.clone());
        }

        match self.cache.fetch(url.clone()) {
            FetchResult::Ok(bytes) => {
                if let Some(is_uploading) = self.is_uploading.read().unwrap().get(&url) {
                    if *is_uploading {
                        return None;
                    }
                }
                let is_on_gpu = self.is_on_gpu.clone();
                let is_uploading = self.is_uploading.clone();
                let url2 = url.clone();
                let ctx = self.ctx.clone();
                std::thread::spawn(move || {
                    let image = Arc::new(
                        egui_extras::RetainedImage::from_image_bytes(url2.clone(), &bytes).unwrap(),
                    );
                    is_on_gpu.write().unwrap().insert(url2.clone(), image);
                    is_uploading.write().unwrap().insert(url2, false);
                    ctx.request_repaint();
                });
                self.is_uploading.write().unwrap().insert(url, true);
            }
            FetchResult::Error(_) => return None,
            FetchResult::Loading => return None,
        }

        None
    }
}
