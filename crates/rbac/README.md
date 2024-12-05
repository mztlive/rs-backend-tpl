# RBAC Crate

## 简介

`rbac` crate 实现了基于角色的访问控制（RBAC）系统，采用 Actor 模式处理并发的权限检查请求。通过与 `database` crate 的紧密集成，提供高效且安全的权限管理解决方案。

## 主要功能

- **权限检查**: 通过 Actor 模式处理并发的权限请求，确保系统的高效性和安全性。
- **策略管理**: 支持动态加载和重置权限策略。
- **集成 Casbin**: 利用 [Casbin](https://github.com/casbin/casbin) 进行权限管理和策略执行。
- **错误处理**: 统一的错误类型，便于调试和错误传播。

## 安装与使用

### 环境要求

- Rust 1.56 及以上版本
- Casbin 库

### 使用方式

1. 在 `Cargo.toml` 中添加依赖：
    ```toml
    [dependencies]
    rbac = { path = "../rbac" }
    casbin = { version = "2.5.0", features = ["tokio"] }
    tokio = { version = "1.0", features = ["full"] }
    ```

2. 初始化 RBAC Actor：
    ```rust
    use rbac::ActorHandler;
    use database::repositories::{RoleRepository, AdminRepository};
    use database::mongodb::connect;
    use config::Config;

    #[tokio::main]
    async fn main() -> rbac::Result<()> {
        let config = Config::from_args().await?;
        let (_client, db) = connect(&config.database.uri, &config.database.db_name).await?;
        
        let role_repo = RoleRepository::new(db.clone());
        let user_repo = AdminRepository::new(db);
        
        let rbac = ActorHandler::new(role_repo, user_repo).await;
        // 继续其他操作
        Ok(())
    }
    ```

3. 检查权限：
    ```rust
    let has_permission = rbac.check_permission(user, "GET", "/admins").await?;
    if has_permission {
        // 允许访问
    } else {
        // 拒绝访问
    }
    ```

## 贡献

欢迎提交问题和合并请求。请确保在提交之前运行所有测试并遵循项目的代码风格。

## 许可证

该项目使用 MIT 许可证。详情请参阅 LICENSE 文件。 