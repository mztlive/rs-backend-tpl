//! Base repository module providing MongoDB database operations
//!
//! This module contains core repository traits and implementations for MongoDB operations.
//! It provides a generic repository interface with common CRUD operations, pagination,
//! filtering and optimistic locking support.

/// Provides error and result types for database operations
use crate::errors::{Error, Result};
use async_trait::async_trait;
use entity_core::{HasId, HasVersion};
use futures_util::StreamExt;
use mongodb::{
    bson::{doc, to_bson, Document},
    Cursor, Database,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// Represents a paginated collection of items
///
/// This struct is used to return paginated results from database queries,
/// containing both the actual items and the total count of matching records.
///
/// # Type Parameters
///
/// * `T` - The type of items in the collection
#[derive(Debug, Serialize, Deserialize)]
pub struct Collection<T> {
    /// The items in the current page of the collection
    pub items: Vec<T>,
    /// Total count of all items matching the query criteria
    pub total: i64,
}

/// Converts a MongoDB cursor into a vector of items
///
/// # Type Parameters
///
/// * `T` - The type of items to collect from the cursor
///
/// # Arguments
///
/// * `cursor` - MongoDB cursor to convert
///
/// # Returns
///
/// Returns a Result containing a Vec of items or an error
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

/// Defines filter behavior for database queries
///
/// This trait should be implemented by types that provide filtering criteria
/// for database queries.
pub trait IFilter {
    /// Converts the filter to a MongoDB document
    ///
    /// # Returns
    ///
    /// A MongoDB Document representing the filter criteria
    fn to_doc(&self) -> Document;
}

/// Defines pagination behavior for database queries
///
/// This trait should be implemented by types that provide pagination parameters
/// for database queries.
pub trait IPaginator {
    /// Returns number of items to skip
    ///
    /// # Returns
    ///
    /// The number of documents to skip in the result set
    fn skip(&self) -> u64;

    /// Returns maximum number of items to return
    ///
    /// # Returns
    ///
    /// The maximum number of documents to return
    fn limit(&self) -> i64;
}

/// Base repository trait providing common database operations
///
/// This trait defines a standard interface for repository implementations,
/// providing common CRUD operations and search functionality.
///
/// # Type Parameters
///
/// * `T` - The entity type this repository manages
#[async_trait]
pub trait IRepository<T>
where
    T: Serialize + Send + Sync + DeserializeOwned,
{
    /// Gets the MongoDB collection name
    ///
    /// # Returns
    ///
    /// The name of the MongoDB collection for this repository
    fn get_collection_name(&self) -> &str;

    /// Gets the MongoDB database instance
    ///
    /// # Returns
    ///
    /// Reference to the MongoDB database instance
    fn get_database(&self) -> &Database;

    /// Creates a new entity in the database
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity to create
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    async fn create(&self, entity: &T) -> Result<()> {
        self.get_database()
            .collection::<T>(self.get_collection_name())
            .insert_one(entity)
            .await?;
        Ok(())
    }

    /// Finds an entity by its ID
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the entity to find
    ///
    /// # Returns
    ///
    /// Optional entity if found
    async fn find_by_id(&self, id: &str) -> Result<Option<T>> {
        let entity = self
            .get_database()
            .collection::<T>(self.get_collection_name())
            .find_one(doc! { "id": id, "deleted_at": 0 })
            .await?;
        Ok(entity)
    }

    /// Finds multiple entities by their IDs
    ///
    /// # Arguments
    ///
    /// * `ids` - Array of entity IDs to find
    ///
    /// # Returns
    ///
    /// Vector of found entities
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

    /// Updates an entity with optimistic locking
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity to update
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    ///
    /// # Errors
    ///
    /// Returns OptimisticLockingError if version mismatch occurs
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

    /// Finds all non-deleted entities
    ///
    /// # Returns
    ///
    /// Vector of all active entities
    async fn find_all(&self) -> Result<Vec<T>> {
        let cursor = self
            .get_database()
            .collection::<T>(self.get_collection_name())
            .find(doc! { "deleted_at": 0 })
            .await?;

        cursor_to_vec(cursor).await
    }

    /// Searches entities with pagination
    ///
    /// # Arguments
    ///
    /// * `filter` - Filter and pagination criteria
    ///
    /// # Returns
    ///
    /// Collection containing matched items and total count
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

    /// Searches a slice of entities based on filter and pagination
    ///
    /// # Arguments
    ///
    /// * `filter` - Filter and pagination criteria
    ///
    /// # Returns
    ///
    /// Vector of matched entities
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

    /// Counts total number of entities matching a filter
    ///
    /// # Arguments
    ///
    /// * `filter` - Filter criteria
    ///
    /// # Returns
    ///
    /// Total count of matching documents
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
