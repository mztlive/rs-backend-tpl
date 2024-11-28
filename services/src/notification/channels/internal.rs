use async_trait::async_trait;
use entities::InternalMessage;

use super::MessageSender;
use crate::{errors::Result, internal_message::IInternalMessageRepository};

pub struct InternalMessageSender<T: IInternalMessageRepository> {
    repo: T,
}

impl<T: IInternalMessageRepository> InternalMessageSender<T> {
    pub fn new(repo: T) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<T: IInternalMessageRepository> MessageSender for InternalMessageSender<T> {
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
