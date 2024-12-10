use crate::errors::Error;
use crate::repositories::base::cursor_to_vec;
use async_trait::async_trait;
use entities::product::Category;
use mongodb::{bson::doc, Database};
use services::errors::Result as ServiceResult;
use services::product::ICategoryRepository;

use super::super::collection_names::CATEGORY;
use super::super::IRepository;

pub struct CategoryRepository {
    pub coll_name: String,
    database: Database,
}

impl CategoryRepository {
    pub fn new(database: Database) -> Self {
        Self {
            coll_name: CATEGORY.to_string(),
            database,
        }
    }
}

impl IRepository<Category> for CategoryRepository {
    fn get_collection_name(&self) -> &str {
        &self.coll_name
    }

    fn get_database(&self) -> &Database {
        &self.database
    }
}

#[async_trait]
impl ICategoryRepository for CategoryRepository {
    async fn create(&self, category: &Category) -> ServiceResult<()> {
        Ok(IRepository::create(self, category).await?)
    }

    async fn update(&self, category: &Category) -> ServiceResult<()> {
        Ok(IRepository::update(self, category).await?)
    }

    async fn find_by_id(&self, id: &str) -> ServiceResult<Option<Category>> {
        Ok(IRepository::find_by_id(self, id).await?)
    }

    async fn find_all(&self) -> ServiceResult<Vec<Category>> {
        Ok(IRepository::find_all(self).await?)
    }

    async fn find_children(&self, parent_id: &str) -> ServiceResult<Vec<Category>> {
        let cursor = self
            .database
            .collection::<Category>(self.get_collection_name())
            .find(doc! {
                "parent_id": parent_id,
                "deleted_at": 0
            })
            .await
            .map_err(|e| Error::DatabaseError(e))?;

        Ok(cursor_to_vec(cursor).await?)
    }
} 