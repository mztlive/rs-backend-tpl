# Database Crate

## 简介

`database` crate 负责与 MongoDB 数据库进行交互，实现了仓储模式（Repository Pattern）来管理各类实体的数据访问。通过抽象的接口和具体的实现，该 crate 提供了统一的数据操作方法，确保业务逻辑与数据访问的解耦。

## 主要功能

- **仓储模式**: 每个实体类型对应一个仓储，统一的数据访问接口。
- **异步操作支持**: 基于 `async/await`，实现非阻塞的数据库操作。
- **错误处理**: 统一的错误处理机制，便于调试和错误传播。
- **RBAC 集成**: 与 `rbac` crate 集成，实现基于角色的权限控制。

## 目录结构

- `repositories/`: 各类实体的仓储实现。
- `mongodb.rs`: MongoDB 连接管理。
- `errors.rs`: 统一错误处理。
- `lib.rs`: 模块导出。

## 安装与运行

### 环境要求

- Rust 1.56 及以上版本
- MongoDB 数据库

### 使用方式

1. 在 `Cargo.toml` 中添加依赖：
    ```toml
    [dependencies]
    database = { path = "../database" }
    mongodb = "3.1.0"
    ```

2. 初始化数据库连接：
    ```rust
    use database::mongodb::connect;
    use config::Config;

    #[tokio::main]
    async fn main() -> database::Result<()> {
        let config = Config::from_args().await?;
        let (client, db) = connect(&config.database.uri, &config.database.db_name).await?;
        // 继续其他操作
        Ok(())
    }
    ```

## 贡献

欢迎提交问题和合并请求。请确保在提交之前运行所有测试并遵循项目的代码风格。

## 许可证

该项目使用 MIT 许可证。详情请参阅 LICENSE 文件。
