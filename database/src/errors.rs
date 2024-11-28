use mongodb::bson::{self, document};
use services::errors::Error as ServiceError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("database error: {0}")]
    DatabaseError(#[from] mongodb::error::Error),

    #[error("bson error: {0}")]
    BsonError(#[from] bson::ser::Error),

    #[error("can not read value from document: {0}")]
    AccessValueError(#[from] document::ValueAccessError),

    #[error("optimistic locking error")]
    OptimisticLockingError,
}

pub type Result<T> = std::result::Result<T, Error>;

// 将数据库错误转换为服务层错误
impl From<Error> for ServiceError {
    fn from(value: Error) -> Self {
        ServiceError::RepositoryError(value.to_string())
    }
}
