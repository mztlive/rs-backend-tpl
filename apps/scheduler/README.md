# Task Crate

## 简介

`task` crate 负责管理和调度项目中的后台任务，使用了 `tokio-cron-scheduler` 来实现定时任务。通过定义不同的任务结构体，实现定期执行的业务逻辑，如数据清理、消息重试等。

## 主要功能

- **任务调度**: 基于 Cron 表达式，支持灵活的任务执行计划。
- **多任务支持**: 能够同时管理和调度多个不同类型的任务。
- **异步执行**: 利用 `tokio` 实现高效的异步任务处理。
- **日志记录**: 记录任务的执行状态，便于监控和调试。

## 安装与使用

### 环境要求

- Rust 1.56 及以上版本
- Tokio 运行时
- MongoDB 数据库

### 使用方式

1. 在 `Cargo.toml` 中添加依赖：
    ```toml
    [dependencies]
    task = { path = "../task" }
    tokio-cron-scheduler = "0.13.0"
    log = "0.4"
    anyhow = "1.0"
    mongodb = "3.1.0"
    database = { path = "../database" }
    services = { path = "../services" }
    entities = { path = "../entities" }
    libs = { path = "../libs" }
    config = { path = "../config" }
    ```

2. 定义任务：
    ```rust
    use task::tasks::CleanupTask;
    use task::tasks::MessageSendTask;
    use task::add_task;
    use tokio_cron_scheduler::JobScheduler;
    use mongodb::Database;

    #[tokio::main]
    async fn main() -> anyhow::Result<()> {
        let scheduler = JobScheduler::new().await?;
        let database: Database = // 初始化数据库连接

        // 添加清理任务
        add_task(&scheduler, CleanupTask).await?;

        // 添加消息重试任务
        add_task(&scheduler, MessageSendTask::new(database.clone(), MessageType::Failed)).await?;
        add_task(&scheduler, MessageSendTask::new(database.clone(), MessageType::UnSent)).await?;

        scheduler.start().await?;
        Ok(())
    }
    ```

## 贡献

欢迎提交问题和合并请求。请确保在提交之前运行所有测试并遵循项目的代码风格。

## 许可证

该项目使用 MIT 许可证。详情请参阅 LICENSE 文件。 