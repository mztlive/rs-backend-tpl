use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::{app_state::AppState, jwt::Engine};

use super::{
    response::{api_permission_denied, api_system_error, api_unauthorized},
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
        Err(err) => return api_system_error(format!("Failed to create jwt engine: {}", err)).into_response(),
    };

    let token = match request.headers().get("Authorization") {
        Some(token) => {
            if let Err(_) = token.to_str() {
                return api_unauthorized().into_response();
            }

            let token = token.to_str().unwrap(); // the unwrap is safe
            token.trim_start_matches("Bearer ")
        }
        None => return api_unauthorized().into_response(),
    };

    match jwt_engine.verify_token(token) {
        Ok(payload) => {
            request.extensions_mut().insert(UserID(payload.id));
            request.extensions_mut().insert(Account(payload.account));
            next.run(request).await
        }
        Err(_) => return api_unauthorized().into_response(),
    }
}

/// RBAC权限控制中间件
///
/// # 功能
/// - 从请求扩展中获取用户账号
/// - 检查用户是否有权限访问当前路径
///
/// # 参数
/// - state: 应用状态
/// - request: HTTP请求
/// - next: 下一个处理器
///
/// # 返回
/// - 如果有权限,继续处理请求
/// - 如果无权限,返回权限拒绝错误
/// - 如果检查过程出错,返回系统错误
pub async fn rbac(State(state): State<AppState>, request: Request, next: Next) -> Response {
    let uname = match request.extensions().get::<Account>() {
        Some(uid) => uid.to_owned(),
        None => return api_unauthorized().into_response(),
    };

    let is_permission = state
        .rbac
        .check_permission(uname.0, request.uri().path().to_string())
        .await;

    match is_permission {
        Ok(is_ok) => {
            if is_ok {
                return next.run(request).await;
            }
        }
        Err(err) => return api_system_error(err.to_string()).into_response(),
    }

    api_permission_denied().into_response()
}
