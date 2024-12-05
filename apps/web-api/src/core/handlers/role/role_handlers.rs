use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    app_state::AppState,
    core::{errors::Result, response::ApiResponse},
};

use super::types::{CreateRoleRequest, RoleItem, UpdateRoleRequest};

pub async fn create_role(State(state): State<AppState>, Json(req): Json<CreateRoleRequest>) -> Result<()> {
    state
        .service_factory()
        .role_service()
        .create_role(req.into())
        .await?;

    // 重新加载RBAC策略
    state.rbac().reset().await?;

    ApiResponse::<()>::ok()
}

pub async fn get_role_list(State(state): State<AppState>) -> Result<Vec<RoleItem>> {
    let roles = state.service_factory().role_service().get_role_list().await?;

    let items = roles.into_iter().map(|role| role.into()).collect();

    ApiResponse::ok_with_data(items)
}

pub async fn update_role(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateRoleRequest>,
) -> Result<()> {
    state
        .service_factory()
        .role_service()
        .update_role(req.to_params(id))
        .await?;

    // 重新加载RBAC策略
    state.rbac().reset().await?;

    ApiResponse::<()>::ok()
}

pub async fn delete_role(State(state): State<AppState>, Path(id): Path<String>) -> Result<()> {
    state.service_factory().role_service().delete_role(id).await?;

    // 重新加载RBAC策略
    state.rbac().reset().await?;

    ApiResponse::<()>::ok()
}
