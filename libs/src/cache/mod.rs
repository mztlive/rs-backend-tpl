use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;
use thiserror::Error;
pub mod memory;

#[derive(Error, Debug)]
pub enum CacheError {
    #[error("序列化错误: {0}")]
    SerializationError(String),
    #[error("反序列化错误: {0}")]
    DeserializationError(String),
    #[error("键不存在")]
    KeyNotFound,
    #[error("其他错误: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, CacheError>;

/// 缓存接口trait
#[async_trait]
pub trait Cache: Send + Sync + Clone {
    async fn get_raw(&self, key: &str) -> Result<Option<Vec<u8>>>;
    async fn set_raw(&self, key: &str, value: Vec<u8>, ttl: Option<u64>) -> Result<()>;

    /// 删除缓存
    async fn delete(&self, key: &str) -> Result<bool>;

    /// 检查键是否存在
    async fn exists(&self, key: &str) -> Result<bool>;

    /// 设置过期时间
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
