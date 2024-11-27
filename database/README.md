# Database Module

数据库访问层模块，负责所有与数据库交互的操作。

## 设计原则

1. 仓储模式 (Repository Pattern)

   - 每个实体类型对应一个仓储
   - 统一的数据访问接口
   - 隔离业务逻辑与数据访问细节

2. 接口抽象
   - 通过 trait 定义通用的仓储接口
   - 支持不同数据库实现的可扩展性
   - 便于单元测试和模拟

## 核心组件

### 1. 基础设施 (Infrastructure)

- `mongodb.rs`: MongoDB 连接管理
- `errors.rs`: 统一的错误处理
- `lib.rs`: 模块导出

### 2. 仓储基类 (Base Repository)

- `repositories/base.rs`: 定义 `IRepository` trait
- 实现通用的 CRUD 操作
- 乐观锁支持
- 分页查询支持

### 3. 具体仓储实现

- `repositories/user.rs`: 用户仓储
- `repositories/role.rs`: 角色仓储
- 实现特定实体的数据访问逻辑
- 支持 RBAC 权限系统

## 主要特性

1. 统一的错误处理

   - 自定义错误类型
   - 错误转换和传播
   - 友好的错误信息

2. 异步操作支持

   - 基于 async/await
   - 非阻塞 I/O
   - 连接池管理

3. 通用查询功能

   - 条件过滤
   - 分页支持
   - 排序功能

4. 数据完整性
   - 乐观锁并发控制
   - 软删除支持
   - 基础字段自动维护
