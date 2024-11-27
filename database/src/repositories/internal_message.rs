use super::{base::cursor_to_vec, collection_names::INTERNAL_MESSAGE, IRepository};
use entities::InternalMessage;
use mongodb::{bson::doc, Database};

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

    pub async fn find_by_recipient_with_filter(
        &self,
        recipient: &str,
        status: Option<String>,
        skip: u64,
        limit: i64,
    ) -> crate::Result<Vec<InternalMessage>> {
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
            .await?;

        cursor_to_vec(cursor).await
    }

    pub async fn mark_as_read(&self, id: &str, recipient: &str) -> crate::Result<bool> {
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
            .await?;

        Ok(result.modified_count > 0)
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
