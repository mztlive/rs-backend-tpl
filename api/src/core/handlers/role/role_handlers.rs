use axum::{
    extract::{Path, State},
    Json,
};
use database::repositories::{AdminRepository, RoleRepository};
use services::RoleService;

use crate::{
    app_state::AppState,
    core::{errors::Result, response::ApiResponse},
};

use super::types::{CreateRoleRequest, RoleItem, UpdateRoleRequest};

pub async fn create_role(State(state): State<AppState>, Json(req): Json<CreateRoleRequest>) -> Result<()> {
    let role_repo = RoleRepository::new(state.db_state.db.clone());
    let admin_repo = AdminRepository::new(state.db_state.db.clone());
    RoleService::new(role_repo, admin_repo)
        .create_role(req.into())
        .await?;

    // 重新加载RBAC策略
    state.rbac.reset().await?;

    ApiResponse::<()>::ok()
}

pub async fn get_role_list(State(state): State<AppState>) -> Result<Vec<RoleItem>> {
    let role_repo = RoleRepository::new(state.db_state.db.clone());
    let admin_repo = AdminRepository::new(state.db_state.db.clone());
    let roles = RoleService::new(role_repo, admin_repo).get_role_list().await?;

    let items = roles.into_iter().map(|role| role.into()).collect();

    ApiResponse::ok_with_data(items)
}

pub async fn update_role(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateRoleRequest>,
) -> Result<()> {
    let role_repo = RoleRepository::new(state.db_state.db.clone());
    let admin_repo = AdminRepository::new(state.db_state.db.clone());
    RoleService::new(role_repo, admin_repo)
        .update_role(req.to_params(id))
        .await?;

    // 重新加载RBAC策略
    state.rbac.reset().await?;

    ApiResponse::<()>::ok()
}

pub async fn delete_role(State(state): State<AppState>, Path(id): Path<String>) -> Result<()> {
    let role_repo = RoleRepository::new(state.db_state.db.clone());
    let admin_repo = AdminRepository::new(state.db_state.db.clone());
    RoleService::new(role_repo, admin_repo).delete_role(id).await?;

    // 重新加载RBAC策略
    state.rbac.reset().await?;

    ApiResponse::<()>::ok()
}
