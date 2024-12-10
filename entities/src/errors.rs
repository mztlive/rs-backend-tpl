#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    LogicError(String),
}

impl Error {
    pub fn from_str(s: &str) -> Self {
        Error::LogicError(s.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
