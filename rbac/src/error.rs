#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Casbin error: {0}")]
    CasbinError(#[from] casbin::error::Error),

    #[error("Fetcher Error from MongoDB: {0}")]
    DatabaseError(#[from] mongodb::error::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
