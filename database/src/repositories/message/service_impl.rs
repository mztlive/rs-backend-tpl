use super::super::base::{cursor_to_vec, IFilter, IPaginator};
use super::super::IRepository;
use super::MessageRepository;
use crate::errors::Error;
use async_trait::async_trait;
use entities::{Message, MessageStatus};
use mongodb::bson::doc;
use services::errors::Result as ServiceResult;
use services::notification::{IMessageRepository, MessageQuery};

/// Converts MessageQuery into MongoDB query document
///
/// This implementation allows filtering messages based on:
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

/// Implements pagination functionality for message queries
///
/// Handles the calculation of:
/// - Number of documents to skip
/// - Maximum documents per page
impl IPaginator for MessageQuery {
    /// Calculates number of documents to skip based on page number and page size
    ///
    /// # Returns
    /// - u64: Number of documents to skip, always >= 0
    fn skip(&self) -> u64 {
        ((self.page.max(1) - 1) * self.page_size).max(0) as u64
    }

    /// Returns the maximum number of documents per page
    ///
    /// # Returns
    /// - i64: Page size limit
    fn limit(&self) -> i64 {
        self.page_size
    }
}

#[async_trait]
impl IMessageRepository for MessageRepository {
    /// Creates a new message in the database
    ///
    /// # Arguments
    /// * `message` - The message to create
    ///
    /// # Returns
    /// * `ServiceResult<()>` - Success or error result
    async fn create(&self, message: &Message) -> ServiceResult<()> {
        IRepository::create(self, message).await?;
        Ok(())
    }

    /// Updates an existing message in the database
    ///
    /// # Arguments
    /// * `message` - The message with updated fields
    ///
    /// # Returns
    /// * `ServiceResult<()>` - Success or error result
    async fn update(&self, message: &Message) -> ServiceResult<()> {
        IRepository::update(self, message).await?;
        Ok(())
    }

    /// Finds a message by its unique identifier
    ///
    /// # Arguments
    /// * `id` - The message ID to search for
    ///
    /// # Returns
    /// * `ServiceResult<Option<Message>>` - The found message or None if not found
    async fn find_by_id(&self, id: &str) -> ServiceResult<Option<Message>> {
        let message = IRepository::find_by_id(self, id).await?;
        Ok(message)
    }

    /// Retrieves all messages marked as failed
    ///
    /// This includes messages that:
    /// - Have a Failed status
    /// - Are not deleted (deleted_at = 0)
    ///
    /// # Returns
    /// * `ServiceResult<Vec<Message>>` - List of failed messages
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

    /// Retrieves all messages with pending status
    ///
    /// # Returns
    /// * `ServiceResult<Vec<Message>>` - List of pending messages
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

    /// Queries messages based on provided filters and pagination
    ///
    /// # Arguments
    /// * `query` - The MessageQuery containing filter criteria and pagination settings
    ///
    /// # Returns
    /// * `ServiceResult<Vec<Message>>` - List of messages matching the query criteria
    async fn query(&self, query: MessageQuery) -> ServiceResult<Vec<Message>> {
        Ok(IRepository::search_slice(self, &query).await?)
    }
}
