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
pub trait Cache: Send + Sync {
    /// 设置缓存
    async fn set<V: Serialize + Send + Sync>(
        &self,
        key: &str,
        value: &V,
        ttl: Option<Duration>,
    ) -> Result<()>;

    /// 获取缓存
    async fn get<V: DeserializeOwned + Send + Sync>(&self, key: &str) -> Result<Option<V>>;

    /// 删除缓存
    async fn delete(&self, key: &str) -> Result<bool>;

    /// 检查键是否存在
    async fn exists(&self, key: &str) -> Result<bool>;

    /// 设置过期时间
    async fn expire(&self, key: &str, ttl: Duration) -> Result<bool>;
}
