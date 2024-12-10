use async_trait::async_trait;
use crate::errors::Result;
use entities::product::Brand;

#[async_trait]
pub trait IBrandRepository: Send + Sync {
    async fn create(&self, brand: &Brand) -> Result<()>;
    async fn update(&self, brand: &Brand) -> Result<()>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Brand>>;
    async fn find_all(&self) -> Result<Vec<Brand>>;
} 