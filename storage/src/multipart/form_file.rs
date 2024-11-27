use axum::extract::multipart::Field;
use std::path::Path;

use super::error::{Error, Result};

/// 表示从 multipart 表单中提取的文件
#[derive(Debug)]
pub struct FormFile {
    /// 文件名
    pub filename: String,
    /// 文件内容
    pub content: Vec<u8>,
    /// 文件类型
    pub content_type: Option<String>,
}

impl FormFile {
    /// 获取文件扩展名
    pub fn extension(&self) -> Option<String> {
        Path::new(&self.filename)
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_string())
    }

    /// 从 multipart 字段中创建 FormFile
    pub async fn from_field(field: Field<'_>) -> Result<Self> {
        let filename = field.file_name().ok_or(Error::InvalidFilename)?.to_string();

        let content_type = field.content_type().map(|mime| mime.to_string());

        let content = field
            .bytes()
            .await
            .map_err(|e| Error::ReadError(e.to_string()))?
            .to_vec();

        Ok(Self {
            filename,
            content,
            content_type,
        })
    }

    pub async fn unique_name(&self) -> String {
        format!(
            "{}.{}",
            libs::next_id().await,
            self.extension().unwrap_or_else(|| "unknown".to_string())
        )
    }
}
