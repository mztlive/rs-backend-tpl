use axum::{extract::State, Json};

use super::types::{AuthRequest, AuthResponse};
use crate::core::errors::{Error, Result};
use crate::core::response::api_ok_with_data;
use crate::app_state::AppState;
use crate::jwt::Engine;
use database::repositories::user::UserRepository;

pub async fn login(State(state): State<AppState>, Json(request): Json<AuthRequest>) -> Result<AuthResponse> {
    let jwt_engine = Engine::new(state.config.app.secret.clone())?;

    let user = UserRepository::new()
        .find_by_account(&request.account, &state.db_state.db)
        .await?;

    if let Some(user) = user {
        if user.secret.is_match(&request.password) {
            let token = jwt_engine.create_token(user)?;
            return api_ok_with_data(AuthResponse { token });
        }
    }

    Err(Error::BadRequest("用户名或密码错误".to_string()))
}