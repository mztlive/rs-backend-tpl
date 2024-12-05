use super::{Cache, Result};
use async_trait::async_trait;
use dashmap::DashMap;
use serde::Serialize;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

#[derive(Clone)]
struct CacheEntry {
    data: Vec<u8>,
    expires_at: Option<Instant>,
}

#[derive(Clone)]
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
    async fn get_raw(&self, key: &str) -> Result<Option<Vec<u8>>> {
        Ok(self.storage.get(key).map(|entry| entry.data.clone()))
    }

    async fn set_raw(&self, key: &str, value: Vec<u8>, ttl: Option<u64>) -> Result<()> {
        let expires_at = ttl.map(|ttl| Instant::now() + Duration::from_secs(ttl));
        self.storage.insert(
            key.to_string(),
            CacheEntry {
                data: value,
                expires_at,
            },
        );
        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<bool> {
        Ok(self.storage.remove(key).is_some())
    }

    async fn exists(&self, key: &str) -> Result<bool> {
        // clone避免死锁
        let entry = self.storage.get(key).map(|entry| entry.clone());

        if let Some(entry) = entry {
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

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use crate::cache::{CacheWithJson, CacheWithString};

    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_memory_cache_basic_operations() {
        let cache = MemoryCache::new();

        // 测试设置和获取
        cache.set_raw("key1", b"value1".to_vec(), None).await.unwrap();
        let value = cache.get_raw("key1").await.unwrap();
        assert_eq!(value, Some(b"value1".to_vec()));

        // 测试不存在的键
        let value = cache.get_raw("non_existent").await.unwrap();
        assert_eq!(value, None);

        // 测试删除
        assert!(cache.delete("key1").await.unwrap());
        assert!(!cache.exists("key1").await.unwrap());
    }

    #[tokio::test]
    async fn test_memory_cache_expiration() {
        let cache = MemoryCache::new();

        // 设置1秒过期时间
        cache.set_raw("key1", b"value1".to_vec(), Some(1)).await.unwrap();

        // 立即检查应该存在
        assert!(cache.exists("key1").await.unwrap());

        // 等待2秒
        tokio::time::sleep(Duration::from_secs(2)).await;

        // 检查是否已过期
        assert!(!cache.exists("key1").await.unwrap());
    }

    #[tokio::test]
    async fn test_memory_cache_expire_command() {
        let cache = MemoryCache::new();

        // 先设置不过期的值
        cache.set_raw("key1", b"value1".to_vec(), None).await.unwrap();

        // 设置过期时间
        assert!(cache.expire("key1", Duration::from_secs(1)).await.unwrap());

        // 立即检查应该存在
        assert!(cache.exists("key1").await.unwrap());

        // 等待2秒
        tokio::time::sleep(Duration::from_secs(2)).await;

        // 检查是否已过期
        assert!(!cache.exists("key1").await.unwrap());
    }

    #[tokio::test]
    async fn test_memory_cache_with_string() {
        let cache = MemoryCache::new();

        // 测试字符串操作
        cache.set("key1", "测试字符串".to_string(), None).await.unwrap();
        let value = cache.get("key1").await.unwrap();
        assert_eq!(value, Some("测试字符串".to_string()));
    }

    #[tokio::test]
    async fn test_memory_cache_with_json() {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct TestStruct {
            field1: String,
            field2: i32,
        }

        let cache = MemoryCache::new();
        let test_data = TestStruct {
            field1: "测试".to_string(),
            field2: 42,
        };

        // 测试JSON序列化和反序列化
        cache.set_json("key1", &test_data, None).await.unwrap();
        let value: TestStruct = cache.get_json("key1").await.unwrap().unwrap();
        assert_eq!(value, test_data);
    }

    #[tokio::test]
    async fn test_memory_cache_performance() {
        use std::time::Instant;

        let cache = MemoryCache::new();
        let iterations = 1000000;

        // 测试写入性能
        let start = Instant::now();
        for i in 0..iterations {
            let key = format!("key{}", i);
            let value = format!("value{}", i);
            cache.set(&key, value, None).await.unwrap();
        }
        let write_duration = start.elapsed();
        println!(
            "写入{}条数据耗时: {:?}, 平均每条: {:?}",
            iterations,
            write_duration,
            write_duration / iterations as u32
        );

        // 测试读取性能
        let start = Instant::now();
        for i in 0..iterations {
            let key = format!("key{}", i);
            let _value = cache.get(&key).await.unwrap();
        }
        let read_duration = start.elapsed();
        println!(
            "读取{}条数据耗时: {:?}, 平均每条: {:?}",
            iterations,
            read_duration,
            read_duration / iterations as u32
        );

        // 测试并发写入性能
        let start = Instant::now();
        let mut handles = Vec::new();
        for i in 0..iterations {
            let cache = cache.clone();
            let handle = tokio::spawn(async move {
                let key = format!("concurrent_key{}", i);
                let value = format!("value{}", i);
                cache.set(&key, value, None).await.unwrap();
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.await.unwrap();
        }
        let concurrent_write_duration = start.elapsed();
        println!(
            "并发写入{}条数据耗时: {:?}, 平均每条: {:?}",
            iterations,
            concurrent_write_duration,
            concurrent_write_duration / iterations as u32
        );

        // 测试并发读取性能
        let start = Instant::now();
        let mut handles = Vec::new();
        for i in 0..iterations {
            let cache = cache.clone();
            let handle = tokio::spawn(async move {
                let key = format!("concurrent_key{}", i);
                let _value = cache.get(&key).await.unwrap();
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.await.unwrap();
        }
        let concurrent_read_duration = start.elapsed();
        println!(
            "并发读取{}条数据耗时: {:?}, 平均每条: {:?}",
            iterations,
            concurrent_read_duration,
            concurrent_read_duration / iterations as u32
        );
    }

    #[tokio::test]
    async fn test_json_performance() {
        let cache = MemoryCache::new();
        let iterations = 100000;

        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct TestData {
            id: i32,
            name: String,
            value: String,
        }

        // 测试JSON写入性能
        let start = Instant::now();
        for i in 0..iterations {
            let key = format!("json_key{}", i);
            let value = TestData {
                id: i,
                name: format!("name{}", i),
                value: format!("value{}", i),
            };
            cache.set_json(&key, &value, None).await.unwrap();
        }
        let json_write_duration = start.elapsed();
        println!(
            "JSON写入{}条数据耗时: {:?}, 平均每条: {:?}",
            iterations,
            json_write_duration,
            json_write_duration / iterations as u32
        );

        // 测试JSON读取性能
        let start = Instant::now();
        for i in 0..iterations {
            let key = format!("json_key{}", i);
            let _value: TestData = cache.get_json(&key).await.unwrap().unwrap();
        }
        let json_read_duration = start.elapsed();
        println!(
            "JSON读取{}条数据耗时: {:?}, 平均每条: {:?}",
            iterations,
            json_read_duration,
            json_read_duration / iterations as u32
        );

        // 测试并发JSON写入性能
        let start = Instant::now();
        let mut handles = Vec::new();
        for i in 0..iterations {
            let cache = cache.clone();
            let handle = tokio::spawn(async move {
                let key = format!("concurrent_json_key{}", i);
                let value = TestData {
                    id: i,
                    name: format!("name{}", i),
                    value: format!("value{}", i),
                };
                cache.set_json(&key, &value, None).await.unwrap();
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.await.unwrap();
        }
        let concurrent_json_write_duration = start.elapsed();
        println!(
            "并发JSON写入{}条数据耗时: {:?}, 平均每条: {:?}",
            iterations,
            concurrent_json_write_duration,
            concurrent_json_write_duration / iterations as u32
        );

        // 测试并发JSON读取性能
        let start = Instant::now();
        let mut handles = Vec::new();
        for i in 0..iterations {
            let cache = cache.clone();
            let handle = tokio::spawn(async move {
                let key = format!("concurrent_json_key{}", i);
                let _value: TestData = cache.get_json(&key).await.unwrap().unwrap();
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.await.unwrap();
        }
        let concurrent_json_read_duration = start.elapsed();
        println!(
            "并发JSON读取{}条数据耗时: {:?}, 平均每条: {:?}",
            iterations,
            concurrent_json_read_duration,
            concurrent_json_read_duration / iterations as u32
        );
    }
}
