use std::path::Path;

use axum::extract::{Multipart, State};
use services::utils;
use storage::LocalStorage;

use crate::{
    app_state::AppState,
    core::{
        errors::{Error, Result},
        response::api_ok_with_data,
    },
};

use super::types::UploadResponse;

/// 处理文件上传
///
/// # 参数
///
/// * `state` - 应用状态
/// * `multipart` - Multipart 表单数据
///
/// # 返回
///
/// 返回上传后的文件URL
pub async fn upload_file(State(state): State<AppState>, mut multipart: Multipart) -> Result<UploadResponse> {
    let storage = LocalStorage::new(&state.config.get_upload_path())
        .await
        .map_err(|e| Error::Internal(format!("Failed to init storage: {}", e)))?;

    // 获取上传的文件
    let field = match multipart.next_field().await {
        Ok(Some(field)) => field,
        Ok(None) => return Err(Error::BadRequest("No file uploaded".to_string())),
        Err(e) => return Err(Error::BadRequest(format!("Invalid form data: {}", e))),
    };

    // 获取文件名
    let file_name = match field.file_name() {
        Some(name) => name.to_string(),
        None => return Err(Error::BadRequest("No file name".to_string())),
    };

    // 生成唯一文件名
    let unique_name = generate_unique_filename(&file_name).await;

    // 读取文件内容
    let content = field
        .bytes()
        .await
        .map_err(|e| Error::Internal(format!("Failed to read file: {}", e)))?;

    // 保存文件
    storage
        .save(&unique_name, &content)
        .await
        .map_err(|e| Error::Internal(format!("Failed to save file: {}", e)))?;

    // 构建文件URL
    let url = state.config.file_url(&unique_name);

    api_ok_with_data(UploadResponse { url })
}

/// 生成唯一的文件名
///
/// # 参数
///
/// * `file_name` - 原始文件名
///
/// # 返回
///
/// 返回生成的唯一文件名，格式为 "{唯一ID}.{扩展名}"
///
/// # 错误
///
/// 如果生成唯一ID失败，将返回错误
async fn generate_unique_filename(file_name: &str) -> String {
    // 生成唯一文件名
    let extension = match Path::new(file_name).extension() {
        Some(ext) => ext.to_str().unwrap_or("unknown"),
        None => "unknown",
    };

    format!("{}.{}", utils::next_id().await, extension)
}
