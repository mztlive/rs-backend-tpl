use async_trait::async_trait;
use futures_util::StreamExt;
use mongodb::{bson::doc, Database};

use entities::Role;
use rbac::{Error as RBACError, RBACRole, RBACRoleStore, Result as RBACResult};

use super::{collection_names::ROLE, IRepository};

/// 角色仓储结构体
///
/// 负责处理角色相关的数据库操作
///
/// # 字段
///
/// * `coll_name` - MongoDB集合名称
/// * `database` - MongoDB数据库实例
pub struct RoleRepository {
    pub coll_name: String,
    database: Database,
}

impl RoleRepository {
    /// 创建一个新的角色仓储实例
    ///
    /// # 返回值
    ///
    /// 返回一个配置好集合名称的 RoleRepository 实例
    pub fn new(database: Database) -> Self {
        RoleRepository {
            coll_name: ROLE.to_string(),
            database,
        }
    }
}

#[async_trait]
impl RBACRoleStore for RoleRepository {
    /// 获取所有未删除的角色
    ///
    /// # 返回值
    ///
    /// 返回包含所有角色的动态特征对象向量
    async fn find_all(&self) -> RBACResult<Vec<Box<dyn RBACRole>>> {
        let mut items = self
            .database
            .collection::<Role>(self.coll_name.as_str())
            .find(doc! {
                "deleted_at": 0
            })
            .await
            .map_err(|e| RBACError::StoreError(e.to_string()))?;

        let mut out: Vec<Box<dyn RBACRole>> = vec![];

        while let Some(item) = items.next().await {
            let item = item.map_err(|e| RBACError::StoreError(e.to_string()))?;
            out.push(Box::new(item));
        }

        Ok(out)
    }
}

impl IRepository<Role> for RoleRepository {
    fn get_collection_name(&self) -> &str {
        &self.coll_name
    }

    fn get_database(&self) -> &Database {
        &self.database
    }
}
