use async_trait::async_trait;
use axum::extract::Multipart;

use super::{
    error::{Error, Result},
    form_file::FormFile,
};

/// Multipart 表单扩展特征
#[async_trait]
pub trait MultipartExt {
    /// 从表单中提取下一个文件
    async fn extract_file(&mut self) -> Result<Option<FormFile>>;

    /// 从表单中提取所有文件
    async fn extract_files(&mut self) -> Result<Vec<FormFile>>;

    /// 从表单中提取指定字段名的文件
    async fn extract_file_by_name(&mut self, field_name: &str) -> Result<Option<FormFile>>;
}

#[async_trait]
impl MultipartExt for Multipart {
    async fn extract_file(&mut self) -> Result<Option<FormFile>> {
        while let Some(field) = self
            .next_field()
            .await
            .map_err(|e| Error::InvalidFormData(e.to_string()))?
        {
            if field.file_name().is_some() {
                return Ok(Some(FormFile::from_field(field).await?));
            }
        }
        Ok(None)
    }

    async fn extract_files(&mut self) -> Result<Vec<FormFile>> {
        let mut files = Vec::new();
        while let Some(file) = self.extract_file().await? {
            files.push(file);
        }
        Ok(files)
    }

    async fn extract_file_by_name(&mut self, field_name: &str) -> Result<Option<FormFile>> {
        while let Some(field) = self
            .next_field()
            .await
            .map_err(|e| Error::InvalidFormData(e.to_string()))?
        {
            if field.name() == Some(field_name) && field.file_name().is_some() {
                return Ok(Some(FormFile::from_field(field).await?));
            }
        }
        Ok(None)
    }
}
