# Notification 模块

## 简介

`services/src/notification` 模块负责处理系统中的消息通知功能，包括多种消息发送渠道（如 Email、SMS、WebSocket、内部消息）的管理和发送。通过抽象的接口和具体的实现，该模块提供了灵活且可扩展的消息通知解决方案。

## 主要功能

- **多渠道支持**: 支持 Email、SMS、WebSocket 以及内部消息等多种发送渠道。
- **消息管理**: 管理消息的发送状态，包括发送成功、失败及重试机制。
- **接口抽象**: 定义统一的消息发送接口，便于扩展新的消息渠道。
- **错误处理**: 统一的错误类型，便于调试和错误传播。

## 目录结构

- `channels/`: 各类消息发送渠道的具体实现。
- `service.rs`: 消息服务的核心逻辑。
- `types.rs`: 消息相关的数据结构与类型定义。

## 使用方式

1. 在 `Cargo.toml` 中添加依赖：
    ```toml
    [dependencies]
    services = { path = "../services" }
    database = { path = "../database" }
    entities = { path = "../entities" }
    ```

2. 初始化消息服务：
    ```rust
    use services::notification::MessageService;
    use database::mongodb::connect;
    use config::Config;

    #[tokio::main]
    async fn main() -> services::Result<()> {
        let config = Config::from_args().await?;
        let (client, db) = connect(&config.database.uri, &config.database.db_name).await?;

        let message_service = MessageService::new(db).await?;
        // 继续其他操作
        Ok(())
    }
    ```

3. 发送消息：
    ```rust
    use entities::Message;
    use services::notification::SendMessageParams;

    let params = SendMessageParams {
        channel: MessageChannel::Email,
        recipient: "user@example.com".to_string(),
        subject: "Welcome".to_string(),
        content: "Thank you for joining us!".to_string(),
    };

    message_service.send_message(params).await?;
    ```

## 贡献

欢迎提交问题和合并请求。请确保在提交之前运行所有测试并遵循项目的代码风格。

## 许可证

该项目使用 MIT 许可证。详情请参阅 LICENSE 文件。 