use async_trait::async_trait;
use crate::errors::Result;
use entities::product::Category;

#[async_trait]
pub trait ICategoryRepository: Send + Sync {
    async fn create(&self, category: &Category) -> Result<()>;
    async fn update(&self, category: &Category) -> Result<()>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Category>>;
    async fn find_all(&self) -> Result<Vec<Category>>;
    async fn find_children(&self, parent_id: &str) -> Result<Vec<Category>>;
} 