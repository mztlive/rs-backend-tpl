mod tasks;

use anyhow::Result;
use config::Config;
use log::info;
use mongodb::Database;
use tasks::{add_task, cleanup::CleanupTask, message_retry::MessageRetryTask, Task};
use tokio_cron_scheduler::JobScheduler;

async fn add_tasks(scheduler: &JobScheduler, database: Database) -> Result<()> {
    // 添加清理任务
    add_task(scheduler, CleanupTask).await?;

    // 添加消息重试任务
    add_task(scheduler, MessageRetryTask::new(database)).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    libs::logger::init();
    info!("Starting task scheduler...");

    let config = Config::from_args().await?;

    // 初始化数据库连接
    let (_, database) = database::mongodb::connect(&config.database.uri, &config.database.db_name).await?;

    // 创建调度器
    let scheduler = JobScheduler::new().await?;

    // 添加任务
    add_tasks(&scheduler, database).await?;

    // 启动调度器
    scheduler.start().await?;

    tokio::signal::ctrl_c().await?;
    info!("Shutting down task scheduler...");

    Ok(())
}
