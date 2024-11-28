# Multipart 模块

## 简介

`storage/src/multipart` 模块负责处理来自 HTTP multipart 表单的文件上传操作。通过 `MultipartExt` 特征扩展，提供了便捷的方法来提取和管理上传的文件，确保文件处理的高效性和安全性。

## 主要功能

- **文件提取**: 提供从 multipart 表单中提取单个或多个文件的方法。
- **扩展特征**: 使用 `MultipartExt` 特征为 `axum::extract::Multipart` 提供额外功能。
- **文件处理**: 包含 `FormFile` 结构体，封装上传文件的相关信息。

## 使用方式

1. 在 `Cargo.toml` 中添加依赖：
    ```toml
    [dependencies]
    axum = { version = "0.7", features = ["multipart"] }
    async-trait = "0.1"
    serde = { version = "1.0", features = ["derive"] }
    tokio = { version = "1.0", features = ["fs", "io-util"] }
    thiserror = "2.0"
    ```

2. 使用 `MultipartExt` 特征：
    ```rust
    use axum::extract::Multipart;
    use storage::multipart::MultipartExt;
    use storage::LocalStorage;

    async fn upload_handler(mut multipart: Multipart, storage: LocalStorage) -> storage::Result<()> {
        let files = multipart.extract_files().await?;
        for file in files {
            storage.save(&file.filename, &file.content).await?;
        }
        Ok(())
    }
    ```

3. 提取指定字段名的文件：
    ```rust
    async fn specific_upload_handler(mut multipart: Multipart, storage: LocalStorage) -> storage::Result<()> {
        if let Some(file) = multipart.extract_file_by_name("avatar").await? {
            storage.save(&file.filename, &file.content).await?;
        }
        Ok(())
    }
    ```

## 贡献

欢迎提交问题和合并请求。请确保在提交之前运行所有测试并遵循项目的代码风格。

## 许可证

该项目使用 MIT 许可证。详情请参阅 LICENSE 文件。 