use super::errors::Result;
use async_trait::async_trait;

pub trait RBACRole: Send {
    fn to_casbin_policy(&self) -> Vec<Vec<String>>;
    fn get_name(&self) -> String;
    fn check_permission(&self, method: &str, path: &str) -> bool;
}

pub trait RBACUser: Send {
    fn account(&self) -> String;
    fn role_name(&self) -> String;
}

#[async_trait]
pub trait RBACRoleStore: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Box<dyn RBACRole>>>;
}

#[async_trait]
pub trait RBACUserStore: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Box<dyn RBACUser>>>;
}
