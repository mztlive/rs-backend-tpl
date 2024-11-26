#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("系统内部错误: {0}")]
    Internal(String),

    #[error("数据不存在")]
    NotFound,

    #[error(transparent)]
    Repository(#[from] database::errors::Error),

    #[error(transparent)]
    Logic(#[from] entities::errors::Error),
}

impl From<String> for Error {
    fn from(msg: String) -> Self {
        Error::Internal(msg)
    }
}

impl From<&str> for Error {
    fn from(msg: &str) -> Self {
        Error::Internal(msg.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Internal(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
