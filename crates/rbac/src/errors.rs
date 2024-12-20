#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Casbin error: {0}")]
    CasbinError(#[from] casbin::error::Error),

    #[error("Store error: {0}")]
    StoreError(String),
}

pub type Result<T> = std::result::Result<T, Error>;
