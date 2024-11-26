use super::response::ApiResponse;
use crate::jwt;
use axum::{http::StatusCode, response::IntoResponse, Json};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("系统内部错误: {0}")]
    Internal(String),

    #[error("请求参数错误: {0}")]
    BadRequest(String),

    #[error("数据不存在")]
    NotFound,

    #[error("未授权访问")]
    Unauthorized,

    #[error("权限不足")]
    Forbidden,

    #[error(transparent)]
    Repository(#[from] database::errors::Error),

    #[error(transparent)]
    Logic(#[from] entities::errors::Error),

    #[error(transparent)]
    Validation(#[from] validator::ValidationErrors),

    #[error(transparent)]
    Jwt(#[from] jwt::Error),
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

impl From<services::errors::Error> for Error {
    fn from(err: services::errors::Error) -> Self {
        Error::Internal(err.to_string())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (status, success) = match self {
            Error::Internal(_) => (500, false),
            Error::BadRequest(_) => (400, false),
            Error::NotFound => (404, false),
            Error::Unauthorized => (401, false),
            Error::Forbidden => (403, false),
            Error::Validation(_) => (400, false),
            Error::Jwt(_) => (401, false),
            Error::Repository(_) => (500, false),
            Error::Logic(_) => (500, false),
        };

        let body = ApiResponse::<()> {
            status,
            message: self.to_string(),
            data: None,
            success,
        };

        (StatusCode::OK, Json(body)).into_response()
    }
}

pub fn internal_error(msg: impl Into<String>) -> Error {
    Error::Internal(msg.into())
}

pub fn not_found() -> Error {
    Error::NotFound
}

pub type Result<T> = std::result::Result<ApiResponse<T>, Error>;
