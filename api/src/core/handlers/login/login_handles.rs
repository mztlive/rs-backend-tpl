use axum::{extract::State, Json};

use super::types::{AuthRequest, AuthResponse};
use crate::app_state::AppState;
use crate::core::errors::{Error, Result};
use crate::core::response::ApiResponse;
use crate::jwt::Engine;
use database::repositories::user::AdminRepository;

pub async fn login(State(state): State<AppState>, Json(request): Json<AuthRequest>) -> Result<AuthResponse> {
    let jwt_engine = Engine::new(state.config.app.secret.clone())?;

    let admin_repo = AdminRepository::new(state.db_state.db.clone());
    let user = admin_repo.find_by_account(&request.account).await?;

    if let Some(user) = user {
        if user.secret.is_match(&request.password) {
            let token = jwt_engine.create_token(user)?;
            return ApiResponse::ok_with_data(AuthResponse { token });
        }
    }

    Err(Error::BadRequest("用户名或密码错误".to_string()))
}
