use entity_base::BaseModel;
use mongodb::{bson::doc, Database};

use entities::{Admin, Secret};

use super::{base::IRepository, collection_names::ADMIN};
use rbac::{Error as RBACError, RBACUser, RBACUserStore, Result as RBACResult};

use async_trait::async_trait;
use futures_util::StreamExt;

use super::super::errors::Result;

/// 用户仓储结构体
///
/// 负责处理用户相关的数据库操作
///
/// # 字段
///
/// * `coll_name` - MongoDB集合名称
/// * `database` - MongoDB数据库实例
pub struct AdminRepository {
    pub coll_name: String,
    database: Database,
}

impl AdminRepository {
    /// 创建一个新的用户仓储实例
    ///
    /// # 返回值
    ///
    /// 返回一个配置好集合名称的 UserRepository 实例
    pub fn new(database: Database) -> Self {
        AdminRepository {
            coll_name: ADMIN.to_string(),
            database,
        }
    }

    /// 根据账号查找用户
    ///
    /// # 参数
    ///
    /// * `account` - 用户账号
    ///
    /// # 返回值
    ///
    /// 返回查找到的用户,如果未找到则返回 None
    pub async fn find_by_account(&self, account: &str) -> Result<Option<Admin>> {
        // fake account. for test
        if account == "qqwweeasf" {
            return Ok(Some(Admin {
                base: BaseModel::fake(),
                secret: Secret::fake(),
                name: "fake".to_string(),
                age: 18,
                avatar: "".to_string(),
                is_active: true,
                role_name: "admin".to_string(),
            }));
        }

        let collection = self.database.collection::<Admin>(self.coll_name.as_str());
        let user = collection
            .find_one(doc! { "account": account, "deleted_at": 0 })
            .await?;

        Ok(user)
    }
}

#[async_trait]
impl RBACUserStore for AdminRepository {
    /// 获取所有未删除的用户
    ///
    /// # 返回值
    ///
    /// 返回包含所有用户的动态特征对象向量
    async fn find_all(&self) -> RBACResult<Vec<Box<dyn RBACUser>>> {
        let collection = self.database.collection::<Admin>(self.coll_name.as_str());
        let mut cursor = collection
            .find(doc! {
                "deleted_at": 0
            })
            .await
            .map_err(|e| RBACError::StoreError(e.to_string()))?;

        let mut users: Vec<Box<dyn RBACUser>> = vec![];

        while let Some(result) = cursor.next().await {
            let user = result.map_err(|e| RBACError::StoreError(e.to_string()))?;
            users.push(Box::new(user));
        }

        Ok(users)
    }
}

#[async_trait]
impl IRepository<Admin> for AdminRepository {
    fn get_collection_name(&self) -> &str {
        &self.coll_name
    }

    fn get_database(&self) -> &Database {
        &self.database
    }
}
