use axum::extract::{Multipart, State};
use storage::{LocalStorage, MultipartExt};

use crate::{
    app_state::AppState,
    core::{
        errors::{Error, Result},
        response::ApiResponse,
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
    let config = state.config().await?;
    let storage = LocalStorage::new(&config.get_upload_path())
        .await
        .map_err(|e| Error::Internal(format!("Failed to init storage: {}", e)))?;

    // 提取文件
    let file = match multipart.extract_file().await {
        Ok(Some(file)) => file,
        Ok(None) => return Err(Error::BadRequest("No file uploaded".to_string())),
        Err(e) => return Err(Error::BadRequest(e.to_string())),
    };

    // 生成唯一文件名
    let unique_name = file.unique_name().await;

    // 保存文件
    storage
        .save(&unique_name, &file.content)
        .await
        .map_err(|e| Error::Internal(format!("Failed to save file: {}", e)))?;

    // 构建文件URL
    let url = config.file_url(&unique_name);

    ApiResponse::ok_with_data(UploadResponse { url })
}
