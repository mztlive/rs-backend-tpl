use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    app_state::AppState,
    core::{errors::Result, response::ApiResponse},
};

use super::types::{AdminItem, CreateAdminRequest, UpdateAdminRequest, UpdateAdminRoleRequest};
use log::{error, info};
use services::AdminService;

pub async fn create_admin(State(state): State<AppState>, Json(req): Json<CreateAdminRequest>) -> Result<()> {
    info!("Creating new admin: {}", req.account);

    AdminService::new(state.db_state.db.clone())
        .create_admin(req.into())
        .await?;

    info!("Admin created successfully");

    state.rbac.reset().await?;

    ApiResponse::<()>::ok()
}

pub async fn get_admin_list(State(state): State<AppState>) -> Result<Vec<AdminItem>> {
    let users = AdminService::new(state.db_state.db.clone())
        .get_admin_list()
        .await?;

    ApiResponse::ok_with_data(users.into_iter().map(|user| user.into()).collect())
}

pub async fn update_admin(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateAdminRequest>,
) -> Result<()> {
    AdminService::new(state.db_state.db.clone())
        .update_admin(req.to_params(id))
        .await?;

    ApiResponse::<()>::ok()
}

pub async fn delete_admin(State(state): State<AppState>, Path(id): Path<String>) -> Result<()> {
    AdminService::new(state.db_state.db.clone())
        .delete_admin(id)
        .await?;

    // 重新加载RBAC策略
    state.rbac.reset().await?;

    ApiResponse::<()>::ok()
}

pub async fn update_admin_role(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateAdminRoleRequest>,
) -> Result<()> {
    AdminService::new(state.db_state.db.clone())
        .update_admin_role(req.to_params(id))
        .await?;

    // 重新加载RBAC策略
    state.rbac.reset().await?;

    ApiResponse::<()>::ok()
}
