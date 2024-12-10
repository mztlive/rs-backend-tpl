use crate::errors::Error;
use async_trait::async_trait;
use entities::product::Supplier;
use mongodb::Database;
use services::errors::Result as ServiceResult;
use services::product::ISupplierRepository;

use super::super::collection_names::SUPPLIER;
use super::super::IRepository;

pub struct SupplierRepository {
    pub coll_name: String,
    database: Database,
}

impl SupplierRepository {
    pub fn new(database: Database) -> Self {
        Self {
            coll_name: SUPPLIER.to_string(),
            database,
        }
    }
}

impl IRepository<Supplier> for SupplierRepository {
    fn get_collection_name(&self) -> &str {
        &self.coll_name
    }

    fn get_database(&self) -> &Database {
        &self.database
    }
}

#[async_trait]
impl ISupplierRepository for SupplierRepository {
    async fn create(&self, supplier: &Supplier) -> ServiceResult<()> {
        Ok(IRepository::create(self, supplier).await?)
    }

    async fn update(&self, supplier: &Supplier) -> ServiceResult<()> {
        Ok(IRepository::update(self, supplier).await?)
    }

    async fn find_by_id(&self, id: &str) -> ServiceResult<Option<Supplier>> {
        Ok(IRepository::find_by_id(self, id).await?)
    }

    async fn find_all(&self) -> ServiceResult<Vec<Supplier>> {
        Ok(IRepository::find_all(self).await?)
    }
} 