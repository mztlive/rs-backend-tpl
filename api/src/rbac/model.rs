use async_trait::async_trait;
use mongodb::Database;

pub trait RBACRole: Send {
    fn to_casbin_policy(&self) -> Vec<Vec<String>>;
}

pub trait RBACUser: Send {
    fn account(&self) -> String;
    fn role_name(&self) -> String;
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Casbin error: {0}")]
    CasbinError(#[from] casbin::error::Error),

    #[error("Fetcher Error from MongoDB: {0}")]
    DatabaseError(#[from] mongodb::error::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[async_trait]
pub trait RBACRoleFetcher: Send {
    async fn find_all(&self, database: &Database) -> Result<Vec<Box<dyn RBACRole>>>;
}

#[async_trait]
pub trait RBACUserFetcher: Send {
    async fn find_all(&self, database: &Database) -> Result<Vec<Box<dyn RBACUser>>>;
}
