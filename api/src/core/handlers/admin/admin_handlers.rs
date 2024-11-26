use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    app_state::AppState,
    core::{
        errors::{Error, Result},
        response::{api_ok, api_ok_with_data},
    },
};

use super::types::{AdminItem, CreateAdminRequest, UpdateAdminRequest, UpdateAdminRoleRequest};
use services::AdminService;

pub async fn create_admin(State(state): State<AppState>, Json(req): Json<CreateAdminRequest>) -> Result<()> {
    let service = AdminService::new();

    service
        .create_admin(
            req.account,
            req.password,
            req.name,
            req.role_name,
            &state.db_state.db,
        )
        .await
        .map_err(|e| Error::BadRequest(e.to_string()))?;

    // 重新加载RBAC策略
    state.rbac.reset().await.map_err(|e| Error::Internal(e))?;

    api_ok()
}

pub async fn get_admin_list(State(state): State<AppState>) -> Result<Vec<AdminItem>> {
    let service = AdminService::new();
    let users = service.get_admin_list(&state.db_state.db).await?;

    let responses = users.into_iter().map(|user| user.into()).collect();

    api_ok_with_data(responses)
}

pub async fn update_admin(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateAdminRequest>,
) -> Result<()> {
    let service = AdminService::new();

    service
        .update_admin(id, req.name, req.password, req.role_name, &state.db_state.db)
        .await
        .map_err(|e| Error::BadRequest(e.to_string()))?;

    api_ok()
}

pub async fn delete_admin(State(state): State<AppState>, Path(id): Path<String>) -> Result<()> {
    AdminService::new().delete_admin(id, &state.db_state.db).await?;

    // 重新加载RBAC策略
    state.rbac.reset().await.map_err(|e| Error::Internal(e))?;

    api_ok()
}

pub async fn update_admin_role(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateAdminRoleRequest>,
) -> Result<()> {
    AdminService::new()
        .update_admin_role(id, req.role_name, &state.db_state.db)
        .await?;

    // 重新加载RBAC策略
    state.rbac.reset().await.map_err(|e| Error::Internal(e))?;

    api_ok()
}
