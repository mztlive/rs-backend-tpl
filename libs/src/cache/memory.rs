use super::{Cache, CacheError, Result};
use async_trait::async_trait;
use dashmap::DashMap;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::RwLock;

struct CacheEntry {
    data: Vec<u8>,
    expires_at: Option<Instant>,
}

pub struct MemoryCache {
    storage: Arc<DashMap<String, CacheEntry>>,
}

impl MemoryCache {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(DashMap::new()),
        }
    }

    fn is_expired(&self, entry: &CacheEntry) -> bool {
        if let Some(expires_at) = entry.expires_at {
            Instant::now() > expires_at
        } else {
            false
        }
    }
}

#[async_trait]
impl Cache for MemoryCache {
    async fn set<V: Serialize + Send + Sync>(
        &self,
        key: &str,
        value: &V,
        ttl: Option<Duration>,
    ) -> Result<()> {
        let serialized =
            serde_json::to_vec(value).map_err(|e| CacheError::SerializationError(e.to_string()))?;

        let expires_at = ttl.map(|duration| Instant::now() + duration);

        self.storage.insert(
            key.to_string(),
            CacheEntry {
                data: serialized,
                expires_at,
            },
        );

        Ok(())
    }

    async fn get<V: DeserializeOwned + Send + Sync>(&self, key: &str) -> Result<Option<V>> {
        if let Some(entry) = self.storage.get(key) {
            if self.is_expired(&entry) {
                self.storage.remove(key);
                return Ok(None);
            }

            let value = serde_json::from_slice(&entry.data)
                .map_err(|e| CacheError::DeserializationError(e.to_string()))?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    async fn delete(&self, key: &str) -> Result<bool> {
        Ok(self.storage.remove(key).is_some())
    }

    async fn exists(&self, key: &str) -> Result<bool> {
        if let Some(entry) = self.storage.get(key) {
            if self.is_expired(&entry) {
                self.storage.remove(key);
                return Ok(false);
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn expire(&self, key: &str, ttl: Duration) -> Result<bool> {
        if let Some(mut entry) = self.storage.get_mut(key) {
            entry.expires_at = Some(Instant::now() + ttl);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
