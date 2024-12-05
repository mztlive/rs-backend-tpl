use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    app_state::AppState,
    core::{errors::Result, response::ApiResponse},
};

use super::types::{AdminItem, CreateAdminRequest, UpdateAdminRequest, UpdateAdminRoleRequest};

pub async fn create_admin(State(state): State<AppState>, Json(req): Json<CreateAdminRequest>) -> Result<()> {
    state
        .service_factory()
        .admin_service()
        .create_admin(req.into())
        .await?;

    state.rbac().reset().await?;

    ApiResponse::<()>::ok()
}

pub async fn get_admin_list(State(state): State<AppState>) -> Result<Vec<AdminItem>> {
    let users = state.service_factory().admin_service().get_admin_list().await?;

    ApiResponse::ok_with_data(users.into_iter().map(|user| user.into()).collect())
}

pub async fn update_admin(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateAdminRequest>,
) -> Result<()> {
    state
        .service_factory()
        .admin_service()
        .update_admin(req.to_params(id))
        .await?;

    ApiResponse::<()>::ok()
}

pub async fn delete_admin(State(state): State<AppState>, Path(id): Path<String>) -> Result<()> {
    state.service_factory().admin_service().delete_admin(id).await?;

    state.rbac().reset().await?;

    ApiResponse::<()>::ok()
}

pub async fn update_admin_role(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateAdminRoleRequest>,
) -> Result<()> {
    state
        .service_factory()
        .admin_service()
        .update_admin_role(req.to_params(id))
        .await?;

    state.rbac().reset().await?;

    ApiResponse::<()>::ok()
}
