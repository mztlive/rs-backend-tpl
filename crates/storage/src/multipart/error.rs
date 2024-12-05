#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("无效的文件名")]
    InvalidFilename,

    #[error("无效的表单数据: {0}")]
    InvalidFormData(String),

    #[error("读取文件失败: {0}")]
    ReadError(String),

    #[error("字段不存在: {0}")]
    FieldNotFound(String),
}

pub type Result<T> = std::result::Result<T, Error>;
