use crate::errors::Result;
use async_trait::async_trait;
use entities::InternalMessage;

#[async_trait]
pub trait IInternalMessageRepository: Send + Sync {
    async fn create(&self, message: &InternalMessage) -> Result<()>;

    async fn find_by_recipient_with_filter(
        &self,
        recipient: &str,
        status: Option<String>,
        skip: u64,
        limit: i64,
    ) -> Result<Vec<InternalMessage>>;

    async fn mark_as_read(&self, id: &str, recipient: &str) -> Result<bool>;
}
