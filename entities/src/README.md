# Entities Crate

## 简介

`entities` crate 定义了项目中的各类实体（Entity），如管理员、角色、消息、操作日志等。通过与 `entity-base` 和 `entity-derive` crate 的集成，简化了实体结构的定义和实现，提高了代码的可维护性和可扩展性。

## 主要功能

- **实体定义**: 定义了项目中使用的各种实体结构体，包含必要的字段和方法。
- **序列化支持**: 利用 `serde` 实现实体的序列化和反序列化，便于数据的存储和传输。
- **错误处理**: 集成自定义的错误类型，处理实体相关的错误情况。
- **RBAC 集成**: 部分实体集成了 RBAC 权限控制接口，支持权限检查。

## 目录结构

- `auth/`: 用户认证相关的实体。
- `errors.rs`: 实体相关的错误定义。
- `internal_message.rs`: 系统内部消息实体。
- `message.rs`: 消息实体和相关状态。
- `operation_log.rs`: 操作日志实体。
- `role.rs`: 角色和权限相关的实体。
- `time.rs`: 时间相关的实体。
- `user.rs`: 用户实体。

## 使用方式

1. 在 `Cargo.toml` 中添加依赖：
    ```toml
    [dependencies]
    entities = { path = "../entities" }
    entity-base = { path = "../entity-base" }
    entity-derive = { path = "../entity-derive" }
    serde = { version = "1.0", features = ["derive"] }
    ```

2. 使用实体结构：
    ```rust
    use entities::Admin;
    use entity_base::BaseModel;
    use serde::{Serialize, Deserialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Admin {
        #[serde(flatten)]
        pub base: BaseModel,
        pub name: String,
        pub role_name: String,
        // 其他字段
    }

    impl Admin {
        pub fn new(id: String, name: String, role_name: String) -> Self {
            Self {
                base: BaseModel::new(id),
                name,
                role_name,
            }
        }
    }
    ```

## 贡献

欢迎提交问题和合并请求。请确保在提交之前运行所有测试并遵循项目的代码风格。

## 许可证

该项目使用 MIT 许可证。详情请参阅 LICENSE 文件。 