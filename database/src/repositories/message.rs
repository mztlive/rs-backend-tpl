use super::base::cursor_to_vec;
use super::{collection_names::MESSAGE, IRepository};
use crate::errors::Error;
use async_trait::async_trait;
use entities::{Message, MessageStatus};
use mongodb::bson::doc;
use mongodb::Database;
use services::errors::Result as ServiceResult;
use services::notification::IMessageRepository;

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
}

impl IRepository<Message> for MessageRepository {
    fn get_collection_name(&self) -> &str {
        &self.coll_name
    }

    fn get_database(&self) -> &Database {
        &self.database
    }
}

#[async_trait]
impl IMessageRepository for MessageRepository {
    async fn create(&self, message: &Message) -> ServiceResult<()> {
        IRepository::create(self, message).await?;
        Ok(())
    }

    async fn update(&self, message: &Message) -> ServiceResult<()> {
        IRepository::update(self, message).await?;
        Ok(())
    }

    async fn find_by_id(&self, id: &str) -> ServiceResult<Option<Message>> {
        let message = IRepository::find_by_id(self, id).await?;
        Ok(message)
    }

    async fn find_failed_messages(&self) -> ServiceResult<Vec<Message>> {
        let cursor = self
            .database
            .collection::<Message>(self.coll_name.as_str())
            .find(doc! {
                "status": MessageStatus::Failed.to_string(),
                "deleted_at": 0
            })
            .await
            .map_err(|e| Error::DatabaseError(e))?;

        let slice = cursor_to_vec(cursor).await?;
        Ok(slice)
    }

    async fn find_pending_messages(&self) -> ServiceResult<Vec<Message>> {
        let cursor = self
            .database
            .collection::<Message>(self.coll_name.as_str())
            .find(doc! { "status": MessageStatus::Pending.to_string() })
            .await
            .map_err(|e| Error::DatabaseError(e))?;
        let slice = cursor_to_vec(cursor).await?;
        Ok(slice)
    }
}
