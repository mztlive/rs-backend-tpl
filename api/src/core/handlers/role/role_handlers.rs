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
    statics,
};

use database::repositories::{role::RoleRepository, IRepository};
use entities::Role;

use super::types::{CreateRoleRequest, RoleItem, UpdateRoleRequest};

pub async fn create_role(State(state): State<AppState>, Json(req): Json<CreateRoleRequest>) -> Result<()> {
    let repo = RoleRepository::new();

    let id = statics::next_id().await;
    let role = Role::new(id, req.name, req.permissions);

    repo.create(&role, &state.db_state.db).await?;

    // 重新加载RBAC策略
    state.rbac.reset().await.map_err(|e| Error::Internal(e))?;

    api_ok()
}

pub async fn get_role_list(State(state): State<AppState>) -> Result<Vec<RoleItem>> {
    let repo = RoleRepository::new();
    let roles = repo.find_all(&state.db_state.db).await?;

    let responses = roles.into_iter().map(|role| role.into()).collect();

    api_ok_with_data(responses)
}

pub async fn update_role(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateRoleRequest>,
) -> Result<()> {
    let repo = RoleRepository::new();

    let mut role = repo
        .find_by_id(&id, &state.db_state.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    if let Some(name) = req.name {
        role.name = name;
    }

    if let Some(permissions) = req.permissions {
        role.permissions = permissions;
    }

    repo.update(&role, &state.db_state.db).await?;

    // 重新加载RBAC策略
    state.rbac.reset().await.map_err(|e| Error::Internal(e))?;

    api_ok()
}

pub async fn delete_role(State(state): State<AppState>, Path(id): Path<String>) -> Result<()> {
    let repo = RoleRepository::new();

    let mut role = repo
        .find_by_id(&id, &state.db_state.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    role.base.delete();
    repo.update(&role, &state.db_state.db).await?;

    // 重新加载RBAC策略
    state.rbac.reset().await.map_err(|e| Error::Internal(e))?;

    api_ok()
}
