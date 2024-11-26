use async_trait::async_trait;
use mongodb::Database;

use super::error::Result;

pub trait RBACRole: Send {
    fn to_casbin_policy(&self) -> Vec<Vec<String>>;
}

pub trait RBACUser: Send {
    fn account(&self) -> String;
    fn role_name(&self) -> String;
}

#[async_trait]
pub trait RBACRoleStore: Send {
    async fn find_all(&self, database: &Database) -> Result<Vec<Box<dyn RBACRole>>>;
}

#[async_trait]
pub trait RBACUserStore: Send {
    async fn find_all(&self, database: &Database) -> Result<Vec<Box<dyn RBACUser>>>;
}
