use async_trait::async_trait;
use entities::Admin;
use crate::errors::Result;

#[async_trait]
pub trait IAdminRepository: Send + Sync {
    async fn create(&self, admin: &Admin) -> Result<()>;
    async fn update(&self, admin: &Admin) -> Result<()>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Admin>>;
    async fn find_by_account(&self, account: &str) -> Result<Option<Admin>>;
    async fn find_all(&self) -> Result<Vec<Admin>>;
}

#[async_trait]
pub trait IRoleRepository: Send + Sync {
    async fn create(&self, role: &entities::Role) -> Result<()>;
    async fn update(&self, role: &entities::Role) -> Result<()>;
    async fn find_by_id(&self, id: &str) -> Result<Option<entities::Role>>;
    async fn find_all(&self) -> Result<Vec<entities::Role>>;
    async fn exists(&self, name: &str) -> Result<bool>;
} 