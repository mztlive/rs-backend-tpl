pub mod cleanup;
pub mod message_retry;

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use log::{error, info};
use tokio_cron_scheduler::{Job, JobScheduler};

#[async_trait]
pub trait Task: Send + Sync {
    fn name(&self) -> &str;
    fn cron(&self) -> &str;
    async fn execute(&self) -> Result<()>;
}

pub async fn add_task<T: Task + 'static>(scheduler: &JobScheduler, task: T) -> Result<()> {
    let task = Arc::new(task);
    let task_clone = task.clone();
    let job = Job::new_async(task.cron(), move |_uuid, _l| {
        let task = task_clone.clone();
        Box::pin(async move {
            if let Err(e) = task.execute().await {
                error!("Task {} failed: {}", task.name(), e);
            }
        })
    })?;

    scheduler.add(job).await?;
    info!("Added task: {}", task.name());
    Ok(())
}
