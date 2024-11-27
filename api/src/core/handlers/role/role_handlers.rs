use axum::{
    extract::{Path, State},
    Json,
};
use services::RoleService;

use crate::{
    app_state::AppState,
    core::{
        errors::{Error, Result},
        response::ApiResponse,
    },
};

use super::types::{CreateRoleRequest, RoleItem, UpdateRoleRequest};

pub async fn create_role(State(state): State<AppState>, Json(req): Json<CreateRoleRequest>) -> Result<()> {
    RoleService::new(state.db_state.db.clone())
        .create_role(req.into())
        .await?;

    // 重新加载RBAC策略
    state.rbac.reset().await.map_err(|e| Error::Internal(e))?;

    ApiResponse::<()>::ok()
}

pub async fn get_role_list(State(state): State<AppState>) -> Result<Vec<RoleItem>> {
    let roles = RoleService::new(state.db_state.db.clone())
        .get_role_list()
        .await?;

    ApiResponse::ok_with_data(roles.into_iter().map(|role| role.into()).collect())
}

pub async fn update_role(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateRoleRequest>,
) -> Result<()> {
    RoleService::new(state.db_state.db.clone())
        .update_role(req.to_params(id))
        .await?;

    // 重新加载RBAC策略
    state.rbac.reset().await.map_err(|e| Error::Internal(e))?;

    ApiResponse::<()>::ok()
}

pub async fn delete_role(State(state): State<AppState>, Path(id): Path<String>) -> Result<()> {
    RoleService::new(state.db_state.db.clone())
        .delete_role(id)
        .await?;

    // 重新加载RBAC策略
    state.rbac.reset().await.map_err(|e| Error::Internal(e))?;

    ApiResponse::<()>::ok()
}
