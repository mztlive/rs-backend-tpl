# 项目名称

## 简介

该项目是一个基于 Rust 的后端模板，使用了 Axum 框架来构建 RESTful API。项目集成了 RBAC 权限控制、JWT 认证、MongoDB 数据库操作等功能。

## 目录结构

- `api/`: 包含 API 相关的代码，包括路由、处理器、中间件等。
- `entity-base/`: 实体基类库，提供基础模型和实体接口。
- `entity-derive/`: 实体派生宏库，提供自定义派生宏。
- `config.toml`: 项目配置文件。
- `.vscode/`: VSCode 配置文件夹。

## 主要功能

- **用户认证**: 使用 JWT 进行用户认证。
- **权限控制**: 基于角色的访问控制（RBAC）。
- **数据库操作**: 使用 MongoDB 进行数据存储和查询。
- **RESTful API**: 提供标准的 RESTful API 接口。

## 安装与运行

### 环境要求

- Rust 1.56 及以上版本
- MongoDB 数据库

### 安装步骤

1. 克隆项目到本地：

   ```bash
   git clone <repository-url>
   cd <project-directory>
   ```

2. 配置项目：

   根据 `config.toml` 文件中的示例，配置数据库连接和应用程序设置。

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

### 中间件

- **授权中间件**: 验证请求头中的 JWT 令牌。
- **RBAC 中间件**: 检查用户是否有权限访问当前路径。

## 贡献

欢迎提交问题和请求合并。请确保在提交之前运行所有测试并遵循项目的代码风格。

## 许可证

该项目使用 MIT 许可证。详情请参阅 LICENSE 文件。
