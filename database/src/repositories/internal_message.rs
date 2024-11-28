use super::{base::cursor_to_vec, collection_names::INTERNAL_MESSAGE, IRepository};
use crate::errors::Error;
use async_trait::async_trait;
use entities::InternalMessage;
use mongodb::{bson::doc, Database};
use services::errors::Result as ServiceResult;
use services::internal_message::IInternalMessageRepository;

pub struct InternalMessageRepository {
    pub coll_name: String,
    database: Database,
}

impl InternalMessageRepository {
    pub fn new(database: Database) -> Self {
        Self {
            coll_name: INTERNAL_MESSAGE.to_string(),
            database,
        }
    }
}

impl IRepository<InternalMessage> for InternalMessageRepository {
    fn get_collection_name(&self) -> &str {
        &self.coll_name
    }

    fn get_database(&self) -> &Database {
        &self.database
    }
}

#[async_trait]
impl IInternalMessageRepository for InternalMessageRepository {
    async fn create(&self, message: &InternalMessage) -> ServiceResult<()> {
        IRepository::create(self, message).await?;
        Ok(())
    }

    async fn find_by_recipient_with_filter(
        &self,
        recipient: &str,
        status: Option<String>,
        skip: u64,
        limit: i64,
    ) -> ServiceResult<Vec<InternalMessage>> {
        let mut filter = doc! { "recipient": recipient, "deleted_at": 0 };
        if let Some(status) = status {
            filter.insert("status", status);
        }

        let cursor = self
            .database
            .collection::<InternalMessage>(self.coll_name.as_str())
            .find(filter)
            .skip(skip)
            .limit(limit)
            .sort(doc! { "created_at": -1 })
            .await
            .map_err(|e| Error::DatabaseError(e))?;

        let slice = cursor_to_vec(cursor).await?;
        Ok(slice)
    }

    async fn mark_as_read(&self, id: &str, recipient: &str) -> ServiceResult<bool> {
        let result = self
            .database
            .collection::<InternalMessage>(self.coll_name.as_str())
            .update_one(
                doc! {
                    "id": id,
                    "recipient": recipient,
                    "deleted_at": 0
                },
                doc! {
                    "$set": {
                        "status": "Read"
                    }
                },
            )
            .await
            .map_err(|e| Error::DatabaseError(e))?;

        Ok(result.modified_count > 0)
    }
}
