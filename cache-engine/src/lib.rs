use lru::LruCache;
use std::num::NonZeroUsize;
use std::sync::Mutex;

pub struct CacheEngine {
    cache: Mutex<LruCache<String, Vec<u8>>>,
}

impl CacheEngine {
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: Mutex::new(LruCache::new(
                NonZeroUsize::new(capacity).unwrap_or(NonZeroUsize::new(100).unwrap()),
            )),
        }
    }

    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        self.cache.lock().unwrap().get(key).cloned()
    }

    pub fn put(&self, key: String, value: Vec<u8>) {
        self.cache.lock().unwrap().put(key, value);
    }
}
