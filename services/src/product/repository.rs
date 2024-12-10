use super::dto::ProductSearchParams;
use super::ProductDetail;
use crate::errors::Result;
use async_trait::async_trait;
use entities::product::{Brand, Category, Product, Supplier, SKU};

#[async_trait]
pub trait IProductRepository: Send + Sync {
    async fn create(&self, product: &Product) -> Result<()>;
    async fn update(&self, product: &Product) -> Result<()>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Product>>;
    async fn find_all(&self) -> Result<Vec<Product>>;
    async fn find_by_category(&self, category_id: &str) -> Result<Vec<Product>>;
    async fn find_featured(&self) -> Result<Vec<Product>>;
    async fn search(&self, params: &ProductSearchParams) -> Result<(u64, Vec<ProductDetail>)>;
    async fn update_many(&self, products: &[Product]) -> Result<()>;
}

#[async_trait]
pub trait ICategoryRepository: Send + Sync {
    async fn create(&self, category: &Category) -> Result<()>;
    async fn update(&self, category: &Category) -> Result<()>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Category>>;
    async fn find_all(&self) -> Result<Vec<Category>>;
    async fn find_children(&self, parent_id: &str) -> Result<Vec<Category>>;
}

#[async_trait]
pub trait IBrandRepository: Send + Sync {
    async fn create(&self, brand: &Brand) -> Result<()>;
    async fn update(&self, brand: &Brand) -> Result<()>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Brand>>;
    async fn find_all(&self) -> Result<Vec<Brand>>;
}

#[async_trait]
pub trait ISupplierRepository: Send + Sync {
    async fn create(&self, supplier: &Supplier) -> Result<()>;
    async fn update(&self, supplier: &Supplier) -> Result<()>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Supplier>>;
    async fn find_all(&self) -> Result<Vec<Supplier>>;
}
