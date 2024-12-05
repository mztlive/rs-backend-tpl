# Storage Crate

## 简介

`storage` crate 负责处理文件存储相关的操作，支持本地存储和 multipart 表单文件处理。通过抽象的接口和具体的实现，该 crate 提供了统一的文件操作方法，确保文件管理的高效性和安全性。

## 主要功能

- **本地文件存储**: 提供基于文件系统的文件保存、读取和删除功能。
- **Multipart 表单处理**: 支持从 HTTP multipart 表单中提取和处理文件。
- **错误处理**: 统一的错误类型，便于调试和错误传播。

## 目录结构

- `local.rs`: 本地存储实现。
- `multipart/`: multipart 表单相关的模块。
- `error.rs`: 统一错误处理。
- `lib.rs`: 模块导出。

## 安装与使用

### 环境要求

- Rust 1.56 及以上版本

### 使用方式

1. 在 `Cargo.toml` 中添加依赖：
    ```toml
    [dependencies]
    storage = { path = "../storage" }
    tokio = { version = "1.0", features = ["fs", "io-util"] }
    axum = { version = "0.7", features = ["multipart"] }
    thiserror = "2.0"
    ```

2. 初始化本地存储：
    ```rust
    use storage::LocalStorage;

    #[tokio::main]
    async fn main() -> storage::Result<()> {
        let storage = LocalStorage::new("./uploads").await?;
        // 继续其他操作
        Ok(())
    }
    ```

3. 使用文件操作功能：
    ```rust
    // 保存文件
    storage.save("path/to/file.txt", &content).await?;

    // 读取文件
    let content = storage.read("path/to/file.txt").await?;

    // 删除文件
    storage.delete("path/to/file.txt").await?;
    ```

4. 处理 multipart 表单文件：
    ```rust
    use axum::extract::Multipart;
    use storage::MultipartExt;

    async fn upload_handler(mut multipart: Multipart, storage: LocalStorage) -> storage::Result<()> {
        let file = multipart.extract_file().await?;
        if let Some(file) = file {
            storage.save(&file.filename, &file.content).await?;
        }
        Ok(())
    }
    ```

## 贡献

欢迎提交问题和合并请求。请确保在提交之前运行所有测试并遵循项目的代码风格。

## 许可证

该项目使用 MIT 许可证。详情请参阅 LICENSE 文件。 