# Auth 模块

## 简介

`entities/src/auth` 模块定义了与用户认证相关的实体结构，如 `Secret`。该模块负责管理用户的认证信息，包括账号、密码等，确保认证过程的安全性和可靠性。

## 主要功能

- **认证信息管理**: 包含用户账号和密码的定义与管理。
- **密码安全**: 支持密码的哈希存储和验证，确保密码安全。
- **序列化支持**: 利用 `serde` 实现认证信息的序列化和反序列化。

## 目录结构

- `secret.rs`: 定义用户认证信息的结构体和相关方法。

## 使用方式

1. 定义用户认证信息：
    ```rust
    use entities::auth::Secret;

    let secret = Secret::new("user_account".to_string(), "user_password".to_string())?;
    ```

2. 验证密码：
    ```rust
    if secret.is_match("input_password") {
        // 密码匹配
    } else {
        // 密码不匹配
    }
    ```

3. 修改密码：
    ```rust
    secret.change_password("new_password".to_string())?;
    ```

## 贡献

欢迎提交问题和合并请求。请确保在提交之前运行所有测试并遵循项目的代码风格。

## 许可证

该项目使用 MIT 许可证。详情请参阅 LICENSE 文件。 