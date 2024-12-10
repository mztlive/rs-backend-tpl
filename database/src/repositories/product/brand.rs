use crate::errors::Error;
use async_trait::async_trait;
use entities::product::Brand;
use mongodb::Database;
use services::errors::Result as ServiceResult;
use services::product::IBrandRepository;

use super::super::collection_names::BRAND;
use super::super::IRepository;

pub struct BrandRepository {
    pub coll_name: String,
    database: Database,
}

impl BrandRepository {
    pub fn new(database: Database) -> Self {
        Self {
            coll_name: BRAND.to_string(),
            database,
        }
    }
}

impl IRepository<Brand> for BrandRepository {
    fn get_collection_name(&self) -> &str {
        &self.coll_name
    }

    fn get_database(&self) -> &Database {
        &self.database
    }
}

#[async_trait]
impl IBrandRepository for BrandRepository {
    async fn create(&self, brand: &Brand) -> ServiceResult<()> {
        Ok(IRepository::create(self, brand).await?)
    }

    async fn update(&self, brand: &Brand) -> ServiceResult<()> {
        Ok(IRepository::update(self, brand).await?)
    }

    async fn find_by_id(&self, id: &str) -> ServiceResult<Option<Brand>> {
        Ok(IRepository::find_by_id(self, id).await?)
    }

    async fn find_all(&self) -> ServiceResult<Vec<Brand>> {
        Ok(IRepository::find_all(self).await?)
    }
} 