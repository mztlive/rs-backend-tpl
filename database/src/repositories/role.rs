use async_trait::async_trait;
use futures_util::StreamExt;
use mongodb::{bson::doc, Database};

use entities::Role;
use rbac::{RBACRole, RBACRoleStore, Result as RBACResult};

use super::collection_names::ROLE;

/// 角色仓储结构体
///
/// 负责处理角色相关的数据库操作
///
/// # 字段
///
/// * `coll_name` - MongoDB集合名称
pub struct RoleRepository {
    pub coll_name: String,
}

impl RoleRepository {
    /// 创建一个新的角色仓储实例
    ///
    /// # 返回值
    ///
    /// 返回一个配置好集合名称的 RoleRepository 实例
    pub fn new() -> Self {
        RoleRepository {
            coll_name: ROLE.to_string(),
        }
    }
}

#[async_trait]
impl RBACRoleStore for RoleRepository {
    /// 获取所有未删除的角色
    ///
    /// # 参数
    ///
    /// * `database` - MongoDB数据库实例
    ///
    /// # 返回值
    ///
    /// 返回包含所有角色的动态特征对象向量
    async fn find_all(&self, database: &Database) -> RBACResult<Vec<Box<dyn RBACRole>>> {
        let mut items = database
            .collection::<Role>(self.coll_name.as_str())
            .find(doc! {
                "deleted_at": 0
            })
            .await?;

        let mut out: Vec<Box<dyn RBACRole>> = vec![];

        while let Some(item) = items.next().await {
            let item = item?;
            out.push(Box::new(item));
        }

        Ok(out)
    }
}
