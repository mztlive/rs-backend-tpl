use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::{app_state::AppState, jwt::Engine};

use super::super::{
    response::ApiResponse,
    schema::{Account, UserID},
};

/// 授权中间件
///
/// # 功能
/// - 验证请求头中的JWT令牌
/// - 从令牌中提取用户ID和账号信息
/// - 将用户信息注入到请求扩展中
///
/// # 参数
/// - state: 应用状态
/// - request: HTTP请求
/// - next: 下一个处理器
///
/// # 返回
/// - 如果验证成功,继续处理请求
/// - 如果验证失败,返回未授权错误
pub async fn authorization(State(state): State<AppState>, mut request: Request, next: Next) -> Response {
    let jwt_engine = match Engine::new(state.config.app.secret.clone()) {
        Ok(jwt_engine) => jwt_engine,
        Err(err) => {
            return ApiResponse::<()>::system_error(format!("Failed to create jwt engine: {}", err))
                .into_response()
        }
    };

    let token = match request.headers().get("Authorization") {
        Some(token) => {
            if let Err(_) = token.to_str() {
                return ApiResponse::<()>::unauthorized().into_response();
            }

            let token = token.to_str().unwrap(); // the unwrap is safe
            token.trim_start_matches("Bearer ")
        }
        None => return ApiResponse::<()>::unauthorized().into_response(),
    };

    match jwt_engine.verify_token(token) {
        Ok(payload) => {
            request.extensions_mut().insert(UserID(payload.id));
            request.extensions_mut().insert(Account(payload.account));
            next.run(request).await
        }
        Err(_) => return ApiResponse::<()>::unauthorized().into_response(),
    }
}
