use std::fmt::Display;

use super::Task;
use anyhow::Result;
use async_trait::async_trait;
use container::ServiceFactory;
use database::repositories::{InternalMessageRepository, MessageRepository};
use entities::MessageStatus;
use log::{error, info};
use mongodb::Database;
use services::MessageService;

#[derive(Debug)]
pub enum MessageType {
    UnSent,
    Failed,
}

impl Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct MessageSendTask<'a> {
    service_factory: &'a ServiceFactory,
    message_type: MessageType,
}

impl<'a> MessageSendTask<'a> {
    pub fn new(service_factory: &'a ServiceFactory, message_type: MessageType) -> Self {
        Self {
            service_factory,
            message_type,
        }
    }
}

#[async_trait]
impl<'a> Task for MessageSendTask<'a> {
    fn name(&self) -> &str {
        "message_retry"
    }

    fn cron(&self) -> &str {
        "0 */1 * * * *" // 每1分钟执行一次
    }

    async fn execute(&self) -> Result<()> {
        info!("Starting message retry task...");

        let service = self.service_factory.notify_service();
        let messages = match self.message_type {
            MessageType::UnSent => service.get_pending_messages().await?,
            MessageType::Failed => service.get_failed_messages().await?,
        };

        let total = messages.len();
        info!("Found {} {} messages to retry", total, self.message_type);

        let mut success_count = 0;
        for message in messages {
            let message_id = message.base.id.clone();
            match service.retry_message(message).await {
                Ok(_) => {
                    success_count += 1;
                    info!("Successfully retried message {}", message_id);
                }
                Err(e) => {
                    error!("Failed to retry message {}: {}", message_id, e);
                }
            }
        }

        info!(
            "Message retry task completed: {}/{} messages retried successfully",
            success_count, total
        );
        Ok(())
    }
}
