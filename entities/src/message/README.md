# Message 模块

## 简介

`entities/src/message` 模块定义了消息相关的实体结构，如 `Message` 和 `MessageStatus`。该模块负责管理系统中的消息发送、状态跟踪和错误处理，确保消息传递的可靠性和可追溯性。

## 主要功能

- **消息实体**: 定义了消息的基本结构，包括发送渠道、接收者、内容等字段。
- **状态管理**: 管理消息的发送状态，如等待发送、已发送、发送失败等。
- **错误处理**: 记录发送失败的错误信息，支持后续的重试机制。
- **显示支持**: 实现 `Display` trait，方便状态的可读化展示。

## 目录结构

- `message.rs`: 消息实体和状态定义。
- `internal_message.rs`: 系统内部消息实体定义。
- `lib.rs`: 模块导出。

## 使用方式

1. 在 `Cargo.toml` 中添加依赖：
    ```toml
    [dependencies]
    entities = { path = "../entities" }
    entity-core = { path = "../entity-core" }
    entity-macros = { path = "../entity-macros" }
    serde = { version = "1.0", features = ["derive"] }
    ```

2. 使用消息结构：
    ```rust
    use entities::Message;
    use entities::MessageChannel;
    use entities::MessageStatus;

    let message = Message::new(
        "message_id".to_string(),
        MessageChannel::Email,
        "recipient@example.com".to_string(),
        "Welcome".to_string(),
        "Thank you for signing up!".to_string(),
    );

    println!("消息状态: {}", message.status);
    ```

3. 更新消息状态：
    ```rust
    let mut message = Message::new(/* 参数 */);
    message.status = MessageStatus::Sent;
    ```

## 贡献

欢迎提交问题和合并请求。请确保在提交之前运行所有测试并遵循项目的代码风格。

## 许可证

该项目使用 MIT 许可证。详情请参阅 LICENSE 文件。 