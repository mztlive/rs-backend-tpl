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

use database::repositories::{role::RoleRepository, user::AdminRepository, IRepository};
use entities::{Admin, Secret};

use super::types::{AdminItem, CreateAdminRequest, UpdateAdminRequest, UpdateAdminRoleRequest};

pub async fn create_admin(State(state): State<AppState>, Json(req): Json<CreateAdminRequest>) -> Result<()> {
    let repo = AdminRepository::new();

    // 检查账号是否已存在
    if let Some(_) = repo.find_by_account(&req.account, &state.db_state.db).await? {
        return Err(Error::BadRequest("账号已存在".to_string()));
    }

    let secret = Secret::new(req.account.clone(), req.password)?;
    let id = statics::next_id().await;

    let user = Admin::new(id, secret, req.name, req.role_name);

    repo.create(&user, &state.db_state.db).await?;

    // 重新加载RBAC策略
    state.rbac.reset().await.map_err(|e| Error::Internal(e))?;

    api_ok()
}

pub async fn get_admin_list(State(state): State<AppState>) -> Result<Vec<AdminItem>> {
    let repo = AdminRepository::new();
    let users = repo.find_all(&state.db_state.db).await?;

    let responses = users.into_iter().map(|user| user.into()).collect();

    api_ok_with_data(responses)
}

pub async fn update_admin(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateAdminRequest>,
) -> Result<()> {
    let repo = AdminRepository::new();

    let mut user = repo
        .find_by_id(&id, &state.db_state.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    if let Some(name) = req.name {
        user.name = name;
    }

    if let Some(password) = req.password {
        user.secret.change_password(password);
    }

    if let Some(role_name) = req.role_name {
        user.role_name = role_name;
    }

    repo.update(&user, &state.db_state.db).await?;

    // 重新加载RBAC策略
    state.rbac.reset().await.map_err(|e| Error::Internal(e))?;

    api_ok()
}

pub async fn delete_admin(State(state): State<AppState>, Path(id): Path<String>) -> Result<()> {
    let repo = AdminRepository::new();

    let mut user = repo
        .find_by_id(&id, &state.db_state.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    user.base.delete();
    repo.update(&user, &state.db_state.db).await?;

    // 重新加载RBAC策略
    state.rbac.reset().await.map_err(|e| Error::Internal(e))?;

    api_ok()
}

pub async fn update_admin_role(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateAdminRoleRequest>,
) -> Result<()> {
    let repo = AdminRepository::new();

    let mut user = repo
        .find_by_id(&id, &state.db_state.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    // 检查新角色是否存在
    let role_repo = RoleRepository::new();
    let roles = role_repo.find_all(&state.db_state.db).await?;
    let role_exists = roles.iter().any(|role| role.name == req.role_name);

    if !role_exists {
        return Err(Error::BadRequest("指定的角色不存在".to_string()));
    }

    user.role_name = req.role_name;
    repo.update(&user, &state.db_state.db).await?;

    // 重新加载RBAC策略
    state.rbac.reset().await.map_err(|e| Error::Internal(e))?;

    api_ok()
}
