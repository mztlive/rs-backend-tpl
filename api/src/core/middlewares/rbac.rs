use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::app_state::AppState;

use super::super::{
    response::{api_permission_denied, api_system_error, api_unauthorized},
    schema::Account,
};

/// RBAC权限控制中间件
///
/// # 功能
/// - 从请求扩展中获取用户账号
/// - 检查用户是否有权限访问当前路径和方法
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
    let account = match request.extensions().get::<Account>() {
        Some(account) => account.to_owned(),
        None => return api_unauthorized().into_response(),
    };

    let method = request.method().as_str().to_uppercase();
    let path = request.uri().path().to_string();

    let is_permission = state
        .rbac
        .check_permission(account.0, method.to_string(), path)
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
