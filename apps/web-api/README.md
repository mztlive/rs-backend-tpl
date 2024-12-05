# API Crate

## 简介

`api` 是项目的后端服务，基于 [Axum](https://github.com/tokio-rs/axum) 框架构建，提供了 RESTful API 接口。该 crate 负责处理来自客户端的 HTTP 请求，进行业务逻辑处理，并与其他服务如数据库、RBAC 等进行交互。

## 主要功能

- **用户认证与授权**: 集成 JWT 认证和基于角色的访问控制（RBAC）。
- **日志记录**: 记录所有操作行为，包括请求路径、方法、操作者等。
- **中间件支持**: 提供操作日志和 RBAC 权限控制中间件。
- **文件上传**: 支持 multipart 表单文件上传。

## 安装与运行

### 环境要求

- Rust 1.56 及以上版本
- MongoDB 数据库

### 安装步骤

1. 克隆项目并进入 `api` 目录：
    ```bash
    git clone <repository-url>
    cd api
    ```

2. 配置项目：
    根据顶层 `config` crate 的配置文件设置数据库连接和应用程序参数。

3. 构建项目：
    ```bash
    cargo build
    ```

4. 运行项目：
    ```bash
    cargo run
    ```

## 使用说明

### API 端点

- `POST /login`: 用户登录，获取 JWT 令牌。
- `GET /admins`: 获取管理员列表。
- `POST /admins`: 创建新管理员。
- `PUT /admins/:id`: 更新管理员信息。
- `DELETE /admins/:id`: 删除管理员。
- 更多端点请参考源码中的路由定义。

### 中间件

- **操作日志中间件**: 记录每个请求的详细信息。
- **RBAC 中间件**: 根据用户角色控制访问权限。

## 贡献

欢迎提交问题和合并请求。请确保在提交之前运行所有测试并遵循项目的代码风格。

## 许可证

该项目使用 MIT 许可证。详情请参阅 LICENSE 文件。 