use async_trait::async_trait;
use entities::Message;
use crate::errors::Result;

#[async_trait]
pub trait IMessageRepository: Send + Sync {
    async fn create(&self, message: &Message) -> Result<()>;
    async fn update(&self, message: &Message) -> Result<()>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Message>>;
    async fn find_failed_messages(&self) -> Result<Vec<Message>>;
    async fn find_pending_messages(&self) -> Result<Vec<Message>>;
} 