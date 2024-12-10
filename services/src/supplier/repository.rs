use async_trait::async_trait;
use crate::errors::Result;
use entities::product::Supplier;

#[async_trait]
pub trait ISupplierRepository: Send + Sync {
    async fn create(&self, supplier: &Supplier) -> Result<()>;
    async fn update(&self, supplier: &Supplier) -> Result<()>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Supplier>>;
    async fn find_all(&self) -> Result<Vec<Supplier>>;
} 