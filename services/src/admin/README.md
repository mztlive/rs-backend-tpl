# Admin 模块

## 简介

`services/src/admin` 模块负责管理员的管理，包括创建、更新、删除管理员及其角色的分配。通过与 `database` crate 的仓储模式集成，实现了高效且安全的管理员管理功能。

## 主要功能

- **创建管理员**: 添加新的管理员，分配相应的角色。
- **更新管理员**: 修改管理员的信息，如姓名、密码、角色等。
- **删除管理员**: 移除管理员账户，确保数据的一致性和安全性。
- **角色管理**: 分配和管理管理员的角色，集成 RBAC 权限控制。

## 目录结构

- `service.rs`: 管理员服务的核心逻辑。
- `types.rs`: 管理员相关的数据结构与类型定义。

## 使用方式

1. 在 `Cargo.toml` 中添加依赖：
    ```toml
    [dependencies]
    services = { path = "../services" }
    database = { path = "../database" }
    entities = { path = "../entities" }
    ```

2. 使用管理员服务功能：
    ```rust
    use services::admin::AdminService;
    use database::mongodb::connect;
    use config::Config;

    #[tokio::main]
    async fn main() -> services::Result<()> {
        let config = Config::from_args().await?;
        let (client, db) = connect(&config.database.uri, &config.database.db_name).await?;

        let admin_service = AdminService::new(db.clone());

        // 创建管理员
        admin_service.create_admin(CreateAdminParams {
            account: "admin".to_string(),
            password: "secure_password".to_string(),
            name: "Admin User".to_string(),
            role_name: "admin".to_string(),
        }).await?;

        // 更新管理员信息
        admin_service.update_admin(UpdateAdminParams {
            id: "admin_id".to_string(),
            name: Some("New Admin Name".to_string()),
            password: Some("new_password".to_string()),
            role_name: Some("superadmin".to_string()),
        }).await?;

        // 删除管理员
        admin_service.delete_admin("admin_id".to_string()).await?;

        Ok(())
    }
    ```

## 贡献

欢迎提交问题和合并请求。请确保在提交之前运行所有测试并遵循项目的代码风格。

## 许可证

该项目使用 MIT 许可证。详情请参阅 LICENSE 文件。 