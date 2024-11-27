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

use database::repositories::{role::RoleRepository, IRepository};
use entities::Role;

use super::types::{CreateRoleRequest, RoleItem, UpdateRoleRequest};

pub async fn create_role(State(state): State<AppState>, Json(req): Json<CreateRoleRequest>) -> Result<()> {
    RoleService::new()
        .create_role(req.name, req.permissions, &state.db_state.db)
        .await?;

    // 重新加载RBAC策略
    state.rbac.reset().await.map_err(|e| Error::Internal(e))?;

    api_ok()
}

pub async fn get_role_list(State(state): State<AppState>) -> Result<Vec<RoleItem>> {
    let roles = RoleService::new().get_role_list(&state.db_state.db).await?;

    let responses = roles.into_iter().map(|role| role.into()).collect();

    api_ok_with_data(responses)
}

pub async fn update_role(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateRoleRequest>,
) -> Result<()> {
    RoleService::new()
        .update_role(id, req.name, req.permissions, &state.db_state.db)
        .await?;

    // 重新加载RBAC策略
    state.rbac.reset().await.map_err(|e| Error::Internal(e))?;

    api_ok()
}

pub async fn delete_role(State(state): State<AppState>, Path(id): Path<String>) -> Result<()> {
    RoleService::new().delete_role(id, &state.db_state.db).await?;

    // 重新加载RBAC策略
    state.rbac.reset().await.map_err(|e| Error::Internal(e))?;

    api_ok()
}
