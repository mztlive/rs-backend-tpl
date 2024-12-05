//! Cache module providing generic caching functionality
//! 
//! This module defines the core caching interfaces and implementations,
//! supporting different cache backends and serialization methods.

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;
use thiserror::Error;
pub mod memory;

/// Represents possible errors that can occur during cache operations
#[derive(Error, Debug)]
pub enum CacheError {
    /// Error during data serialization
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    /// Error during data deserialization
    #[error("Deserialization error: {0}")]
    DeserializationError(String),
    
    /// Requested key does not exist in cache
    #[error("Key not found")]
    KeyNotFound,
    
    /// Other cache-related errors
    #[error("Other error: {0}")]
    Other(String),
}

/// Type alias for Results from cache operations
pub type Result<T> = std::result::Result<T, CacheError>;

/// Trait defining the core functionality for cache implementations
/// 
/// This trait provides the basic operations that any cache implementation
/// must support, including getting and setting values, deletion, and TTL management.
#[async_trait]
pub trait Cache: Send + Sync + Clone {
    /// Retrieves raw bytes for a given key
    /// 
    /// # Arguments
    /// * `key` - The cache key to retrieve
    async fn get_raw(&self, key: &str) -> Result<Option<Vec<u8>>>;

    /// Stores raw bytes for a given key
    /// 
    /// # Arguments
    /// * `key` - The cache key to store
    /// * `value` - The raw bytes to store
    /// * `ttl` - Optional time-to-live in seconds
    async fn set_raw(&self, key: &str, value: Vec<u8>, ttl: Option<u64>) -> Result<()>;

    /// Deletes a key from the cache
    /// 
    /// # Arguments
    /// * `key` - The cache key to delete
    async fn delete(&self, key: &str) -> Result<bool>;

    /// Checks if a key exists in the cache
    /// 
    /// # Arguments
    /// * `key` - The cache key to check
    async fn exists(&self, key: &str) -> Result<bool>;

    /// Sets expiration time for a key
    /// 
    /// # Arguments
    /// * `key` - The cache key to set expiration for
    /// * `ttl` - The time-to-live duration
    async fn expire(&self, key: &str, ttl: Duration) -> Result<bool>;
}

#[async_trait]
pub trait CacheWithString: Cache {
    async fn get(&self, key: &str) -> Result<Option<String>> {
        let raw = self.get_raw(key).await?;
        if let Some(raw) = raw {
            return Ok(Some(String::from_utf8(raw).map_err(|_| {
                CacheError::DeserializationError("字符串转换错误".to_string())
            })?));
        }

        Ok(None)
    }

    async fn set(&self, key: &str, value: String, ttl: Option<u64>) -> Result<()> {
        self.set_raw(key, value.as_bytes().to_vec(), ttl).await
    }
}

#[async_trait]
pub trait CacheWithJson: Cache {
    async fn get_json<V: DeserializeOwned>(&self, key: &str) -> Result<Option<V>> {
        let raw = self.get_raw(key).await?;
        if let Some(raw) = raw {
            return Ok(Some(serde_json::from_slice(&raw).map_err(|_| {
                CacheError::DeserializationError("JSON转换错误".to_string())
            })?));
        }

        Ok(None)
    }

    async fn set_json<V>(&self, key: &str, value: &V, ttl: Option<u64>) -> Result<()>
    where
        V: Serialize + Send + Sync,
    {
        let raw = serde_json::to_vec(value)
            .map_err(|_| CacheError::SerializationError("JSON转换错误".to_string()))?;

        self.set_raw(key, raw, ttl).await
    }
}

impl<T: Cache> CacheWithString for T {}
impl<T: Cache> CacheWithJson for T {}
