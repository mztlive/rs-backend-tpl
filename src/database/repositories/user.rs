use mongodb::{bson::doc, Database};

use crate::{
    entities::{common::Secret, user::User, BaseModel},
    rbac::{Error, RBACUser, RBACUserFetcher},
};

use super::collection_names::USER;

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
pub struct UserRepository {
    pub coll_name: String,
}

impl UserRepository {
    /// 创建一个新的用户仓储实例
    ///
    /// # 返回值
    ///
    /// 返回一个配置好集合名称的 UserRepository 实例
    pub fn new() -> Self {
        UserRepository {
            coll_name: USER.to_string(),
        }
    }

    /// 根据账号查找用户
    ///
    /// # 参数
    ///
    /// * `account` - 用户账号
    /// * `database` - MongoDB数据库实例
    ///
    /// # 返回值
    ///
    /// 返回查找到的用户,如果未找到则返回 None
    pub async fn find_by_account(&self, account: &str, database: &Database) -> Result<Option<User>> {
        // fake account. for test
        if account == "qqwweeasf" {
            return Ok(Some(User {
                base: BaseModel::fake(),
                secret: Secret::fake(),
                name: "fake".to_string(),
                age: 18,
                avatar: "".to_string(),
                is_active: true,
                role_name: "admin".to_string(),
            }));
        }

        let collection = database.collection::<User>(self.coll_name.as_str());
        let user = collection
            .find_one(doc! { "account": account, "deleted_at": 0 })
            .await?;

        Ok(user)
    }
}

#[async_trait]
impl RBACUserFetcher for UserRepository {
    /// 获取所有未删除的用户
    ///
    /// # 参数
    ///
    /// * `database` - MongoDB数据库实例
    ///
    /// # 返回值
    ///
    /// 返回包含所有用户的动态特征对象向量
    async fn find_all(&self, database: &Database) -> std::result::Result<Vec<Box<dyn RBACUser>>, Error> {
        let collection = database.collection::<User>(self.coll_name.as_str());
        let mut cursor = collection
            .find(doc! {
                "deleted_at": 0
            })
            .await?;

        let mut users: Vec<Box<dyn RBACUser>> = vec![];

        while let Some(result) = cursor.next().await {
            let user = result?;
            users.push(Box::new(user));
        }

        Ok(users)
    }
}
