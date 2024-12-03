mod tasks;

use anyhow::Result;
use config::SafeConfig;
use container::ServiceFactory;
use log::info;
use tasks::{
    add_task,
    cleanup::CleanupTask,
    message_retry::{MessageSendTask, MessageType},
};
use tokio_cron_scheduler::JobScheduler;

async fn add_tasks(scheduler: &'static JobScheduler, service_factory: &'static ServiceFactory) -> Result<()> {
    // 添加清理任务
    add_task(scheduler, CleanupTask).await?;

    // 添加失败消息重试任务
    add_task(
        scheduler,
        MessageSendTask::new(service_factory, MessageType::Failed),
    )
    .await?;

    // 添加未发送的消息重试任务
    add_task(
        scheduler,
        MessageSendTask::new(service_factory, MessageType::UnSent),
    )
    .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    libs::logger::init();
    info!("Starting task scheduler...");

    let config = SafeConfig::from_args().await?;
    let config = config.get_config().await?;

    // 初始化数据库连接
    let (_, database) = database::mongodb::connect(&config.database.uri, &config.database.db_name).await?;

    // 创建调度器并转换为静态引用
    let scheduler = Box::new(JobScheduler::new().await?);
    let scheduler: &'static JobScheduler = Box::leak(scheduler);

    // 创建 service_factory 并转换为静态引用
    let service_factory = Box::new(ServiceFactory::new(database.clone()));
    let service_factory: &'static ServiceFactory = Box::leak(service_factory);

    // 添加任务
    add_tasks(scheduler, service_factory).await?;

    // 启动调度器
    scheduler.start().await?;

    tokio::signal::ctrl_c().await?;
    info!("Shutting down task scheduler...");

    Ok(())
}
