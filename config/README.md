# Config Crate

## 简介

`config` crate 负责管理项目的配置，包括应用参数和数据库连接信息。基于 `clap` 和 `serde` 实现了命令行参数解析和配置文件加载，支持 TOML 格式的配置文件。

## 主要功能

- **配置管理**: 加载和解析配置文件，支持命令行参数覆盖配置。
- **环境支持**: 支持多环境（开发、测试、生产）的配置切换。
- **错误处理**: 统一的错误类型，便于调试和错误传播。

## 使用方式

1. 在 `Cargo.toml` 中添加依赖：
    ```toml
    [dependencies]
    config = { path = "../config" }
    clap = { version = "4.5.21", features = ["derive"] }
    serde = { version = "1.0", features = ["derive"] }
    toml = "0.8"
    tokio = { version = "1.0", features = ["fs"] }
    thiserror = "2.0"
    ```

2. 创建配置文件 `config.toml`：
    ```toml
    [app]
    port = 8080
    secret = "your_secret_key"
    upload_path = "./uploads"

    [database]
    uri = "mongodb://localhost:27017"
    db_name = "your_db"
    ```

3. 加载配置：
    ```rust
    use config::Config;

    #[tokio::main]
    async fn main() -> config::Result<()> {
        let config = Config::from_file("./config.toml").await?;
        println!("App Port: {}", config.app.port);
        Ok(())
    }
    ```

## 贡献

欢迎提交问题和合并请求。请确保在提交之前运行所有测试并遵循项目的代码风格。

## 许可证

该项目使用 MIT 许可证。详情请参阅 LICENSE 文件。 