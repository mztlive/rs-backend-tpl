use super::Task;
use anyhow::Result;
use async_trait::async_trait;
use log::{error, info};
use mongodb::Database;
use services::MessageService;

pub struct MessageRetryTask {
    database: Database,
}

impl MessageRetryTask {
    pub fn new(database: Database) -> Self {
        Self { database }
    }
}

#[async_trait]
impl Task for MessageRetryTask {
    fn name(&self) -> &str {
        "message_retry"
    }

    fn cron(&self) -> &str {
        "0 */1 * * * *" // 每5分钟执行一次
    }

    async fn execute(&self) -> Result<()> {
        info!("Starting message retry task...");

        let service = MessageService::new(self.database.clone());
        let failed_messages = service.get_failed_messages().await?;

        let total = failed_messages.len();
        info!("Found {} failed messages to retry", total);

        let mut success_count = 0;
        for message in failed_messages {
            match service.retry_failed_message(&message.base.id).await {
                Ok(_) => {
                    success_count += 1;
                    info!("Successfully retried message {}", message.base.id);
                }
                Err(e) => {
                    error!("Failed to retry message {}: {}", message.base.id, e);
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
