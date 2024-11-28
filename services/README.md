# Services Crate

## 简介

`services` crate 负责实现项目的核心业务逻辑，封装了各类服务如管理员管理、消息发送、操作日志等。通过与 `database` 和 `rbac` 等 crate 的协作，提供高层次的业务功能接口，供 `api` crate 调用。

## 主要功能

- **管理员服务**: 创建、更新、删除管理员及其角色管理。
- **消息服务**: 处理消息的发送、重试及状态管理，支持多种发送渠道（Email, SMS, WebSocket, 内部消息）。
- **操作日志服务**: 记录系统中的所有操作行为，便于审计和监控。
- **内部消息服务**: 管理系统内部的消息通知。
- **RBAC 集成**: 与 RBAC 权限控制无缝集成，确保权限检查的高效性和安全性。

## 安装与使用

### 环境要求

- Rust 1.56 及以上版本
- MongoDB 数据库

### 使用方式

1. 在 `Cargo.toml` 中添加依赖：
    ```toml
    [dependencies]
    services = { path = "../services" }
    database = { path = "../database" }
    rbac = { path = "../rbac" }
    libs = { path = "../libs" }
    ```

2. 初始化服务：
    ```rust
    use services::{AdminService, MessageService};
    use database::repositories::AdminRepository;
    use mongodb::Database;

    fn main() {
        // 假设已经有 Database 实例
        let db: Database = // 初始化数据库连接
        let admin_service = AdminService::new(db.clone());
        let message_service = MessageService::new(db);
        // 继续其他操作
    }
    ```

3. 使用服务功能：
    ```rust
    // 创建管理员
    admin_service.create_admin(params).await?;

    // 发送消息
    message_service.send_message(params).await?;
    ```

## 贡献

欢迎提交问题和合并请求。请确保在提交之前运行所有测试并遵循项目的代码风格。

## 许可证

该项目使用 MIT 许可证。详情请参阅 LICENSE 文件。 