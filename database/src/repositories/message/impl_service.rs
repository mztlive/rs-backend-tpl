use super::super::base::{cursor_to_vec, IFilter, IPaginator};
use super::super::IRepository;
use super::MessageRepository;
use crate::errors::Error;
use async_trait::async_trait;
use entities::{Message, MessageStatus};
use mongodb::bson::doc;
use services::errors::Result as ServiceResult;
use services::notification::{IMessageRepository, MessageQuery};

impl IFilter for MessageQuery {
    fn to_doc(&self) -> mongodb::bson::Document {
        let mut filter = doc! {
            "deleted_at": 0
        };

        if let Some(channel) = &self.channel {
            filter.insert("channel", channel.to_string());
        }

        if let Some(recipient) = &self.recipient {
            filter.insert("recipient", recipient);
        }

        if let Some(status) = &self.status {
            filter.insert("status", status);
        }

        filter
    }
}

impl IPaginator for MessageQuery {
    fn skip(&self) -> u64 {
        ((self.page.max(1) - 1) * self.page_size).max(0) as u64
    }

    fn limit(&self) -> i64 {
        self.page_size
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
            .get_database()
            .collection::<Message>(self.get_collection_name())
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
            .get_database()
            .collection::<Message>(self.get_collection_name())
            .find(doc! { "status": MessageStatus::Pending.to_string() })
            .await
            .map_err(|e| Error::DatabaseError(e))?;
        let slice = cursor_to_vec(cursor).await?;
        Ok(slice)
    }

    async fn query(&self, query: MessageQuery) -> ServiceResult<Vec<Message>> {
        Ok(IRepository::search_slice(self, &query).await?)
    }
}
