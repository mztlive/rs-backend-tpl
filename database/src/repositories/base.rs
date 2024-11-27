use crate::errors::{Error, Result};
use async_trait::async_trait;
use entity_base::{HasId, HasVersion};
use futures_util::StreamExt;
use mongodb::{
    bson::{doc, to_bson, Document},
    Cursor, Database,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Collection<T> {
    pub items: Vec<T>,
    pub total: i64,
}

/// validate page size
pub fn default_page_size() -> i64 {
    20
}

/// validate page size
pub fn default_page() -> i64 {
    1
}

pub enum NumberItemValueType {
    I64,
    I32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NumberItem {
    pub id: Option<String>,
    pub name: String,
    pub value: i64,
}

impl NumberItem {
    pub fn new(name: String, value: i64, id: Option<String>) -> Self {
        NumberItem { name, value, id }
    }

    /// returns a vector of [NumberItem] from a cursor
    ///
    /// Require a cursor document must with 2 fields: _id and count
    /// _id will be using as name and id for [NumberItem]
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    /// * access value error
    pub async fn vec_from_cursor(
        mut cursor: Cursor<Document>,
        v_type: NumberItemValueType,
    ) -> Result<Vec<NumberItem>> {
        let mut result = vec![];
        while let Some(item) = cursor.next().await {
            let item = item?;
            let name = item.get_str("_id")?;

            let count = match v_type {
                NumberItemValueType::I64 => item.get_i64("count")?,
                NumberItemValueType::I32 => item.get_i32("count")? as i64,
            };

            result.push(NumberItem::new(name.to_string(), count, Some(name.to_string())));
        }

        Ok(result)
    }
}

pub async fn cursor_to_vec<T>(mut cursor: Cursor<T>) -> Result<Vec<T>>
where
    Cursor<T>: futures::stream::StreamExt<Item = std::result::Result<T, mongodb::error::Error>>,
{
    let mut result = vec![];
    while let Some(item) = cursor.next().await {
        result.push(item?);
    }

    Ok(result)
}

pub trait IFilter {
    fn to_doc(&self) -> Document;
}

pub trait IPaginator {
    fn skip(&self) -> u64;

    fn limit(&self) -> i64;
}

#[async_trait]
pub trait IRepository<T>
where
    T: Serialize + Send + Sync + DeserializeOwned,
{
    fn get_collection_name(&self) -> &str;

    fn get_database(&self) -> &Database;

    async fn create(&self, entity: &T) -> Result<()> {
        self.get_database()
            .collection::<T>(self.get_collection_name())
            .insert_one(entity)
            .await?;
        Ok(())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<T>> {
        let entity = self
            .get_database()
            .collection::<T>(self.get_collection_name())
            .find_one(doc! { "id": id, "deleted_at": 0 })
            .await?;
        Ok(entity)
    }

    async fn find_by_ids(&self, ids: &[String]) -> Result<Vec<T>> {
        let mut cursor = self
            .get_database()
            .collection::<T>(self.get_collection_name())
            .find(doc! { "id": { "$in": ids }, "deleted_at": 0 })
            .await?;

        let mut entities = vec![];
        while let Some(entity) = cursor.next().await {
            entities.push(entity?);
        }
        Ok(entities)
    }

    async fn update(&self, entity: &T) -> Result<()>
    where
        T: HasVersion + Serialize + HasId,
    {
        let previous_version = entity.get_version();
        let next_version = previous_version + 1;
        let bson = to_bson(entity)?;

        let mut doc = bson.as_document().unwrap().clone();
        doc.insert("version", next_version as i64);

        let result = self
            .get_database()
            .collection::<T>(self.get_collection_name())
            .update_one(
                doc! {
                    "id": entity.get_id(),
                    "version": previous_version as i64,
                },
                doc! {
                    "$set": doc
                },
            )
            .await?;

        if result.modified_count == 0 {
            return Err(Error::OptimisticLockingError);
        }

        Ok(())
    }

    async fn find_all(&self) -> Result<Vec<T>> {
        let cursor = self
            .get_database()
            .collection::<T>(self.get_collection_name())
            .find(doc! { "deleted_at": 0 })
            .await?;

        cursor_to_vec(cursor).await
    }

    async fn search<F>(&self, filter: &F) -> Result<Collection<T>>
    where
        F: IFilter + IPaginator + Send + Sync,
    {
        let items = self.search_slice(filter).await?;
        let total = self.search_count(filter).await?;

        Ok(Collection {
            items,
            total: total as i64,
        })
    }

    async fn search_slice<F>(&self, filter: &F) -> Result<Vec<T>>
    where
        F: IFilter + IPaginator + Send + Sync,
    {
        let cursor = self
            .get_database()
            .collection::<T>(self.get_collection_name())
            .find(filter.to_doc())
            .skip(filter.skip())
            .limit(filter.limit())
            .sort(doc! { "created_at": -1 })
            .await?;

        cursor_to_vec(cursor).await
    }

    async fn search_count<F>(&self, filter: &F) -> Result<u64>
    where
        F: IFilter + Send + Sync,
    {
        let count = self
            .get_database()
            .collection::<T>(self.get_collection_name())
            .count_documents(filter.to_doc())
            .await?;

        Ok(count)
    }
}
