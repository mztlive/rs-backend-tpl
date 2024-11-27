use async_trait::async_trait;
use database::repositories::{internal_message::InternalMessageRepository, IRepository};
use entities::InternalMessage;
use mongodb::Database;

use super::MessageSender;
use crate::errors::Result;

pub struct InternalMessageSender {
    repo: InternalMessageRepository,
}

impl InternalMessageSender {
    pub fn new(database: Database) -> Self {
        Self {
            repo: InternalMessageRepository::new(database),
        }
    }
}

#[async_trait]
impl MessageSender for InternalMessageSender {
    async fn send(&self, recipient: &str, subject: &str, content: &str) -> Result<()> {
        let message = InternalMessage::new(
            libs::next_id().await,
            recipient.to_string(),
            subject.to_string(),
            content.to_string(),
        );

        self.repo.create(&message).await?;

        Ok(())
    }
}
