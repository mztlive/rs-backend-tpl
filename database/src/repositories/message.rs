use super::base::cursor_to_vec;
use super::{collection_names::MESSAGE, IRepository};
use crate::Result;
use entities::{Message, MessageStatus};
use mongodb::bson::doc;
use mongodb::Database;

pub struct MessageRepository {
    pub coll_name: String,
    database: Database,
}

impl MessageRepository {
    pub fn new(database: Database) -> Self {
        Self {
            coll_name: MESSAGE.to_string(),
            database,
        }
    }

    pub async fn find_failed_messages(&self) -> Result<Vec<Message>> {
        let cursor = self
            .database
            .collection::<Message>(self.coll_name.as_str())
            .find(doc! {
                "status": MessageStatus::Failed.to_string(),
                "deleted_at": 0
            })
            .await?;

        cursor_to_vec(cursor).await
    }

    pub async fn find_pending_messages(&self) -> Result<Vec<Message>> {
        let cursor = self
            .database
            .collection::<Message>(self.coll_name.as_str())
            .find(doc! { "status": MessageStatus::Pending.to_string() })
            .await?;
        cursor_to_vec(cursor).await
    }
}

impl IRepository<Message> for MessageRepository {
    fn get_collection_name(&self) -> &str {
        &self.coll_name
    }

    fn get_database(&self) -> &Database {
        &self.database
    }
}
