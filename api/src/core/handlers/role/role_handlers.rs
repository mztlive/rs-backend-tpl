use axum::{
    extract::{Path, State},
    Json,
};
use services::RoleService;

use crate::{
    app_state::AppState,
    core::{
        errors::{Error, Result},
        response::{api_ok, api_ok_with_data},
    },
};

use super::types::{CreateRoleRequest, RoleItem, UpdateRoleRequest};

pub async fn create_role(State(state): State<AppState>, Json(req): Json<CreateRoleRequest>) -> Result<()> {
    let service = RoleService::new(state.db_state.db.clone());
    service.create_role(req.name, req.permissions).await?;

    // 重新加载RBAC策略
    state.rbac.reset().await.map_err(|e| Error::Internal(e))?;

    api_ok()
}

pub async fn get_role_list(State(state): State<AppState>) -> Result<Vec<RoleItem>> {
    let service = RoleService::new(state.db_state.db.clone());
    let roles = service.get_role_list().await?;

    let responses = roles.into_iter().map(|role| role.into()).collect();

    api_ok_with_data(responses)
}

pub async fn update_role(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateRoleRequest>,
) -> Result<()> {
    let service = RoleService::new(state.db_state.db.clone());
    service.update_role(id, req.name, req.permissions).await?;

    // 重新加载RBAC策略
    state.rbac.reset().await.map_err(|e| Error::Internal(e))?;

    api_ok()
}

pub async fn delete_role(State(state): State<AppState>, Path(id): Path<String>) -> Result<()> {
    let service = RoleService::new(state.db_state.db.clone());
    service.delete_role(id).await?;

    // 重新加载RBAC策略
    state.rbac.reset().await.map_err(|e| Error::Internal(e))?;

    api_ok()
}
