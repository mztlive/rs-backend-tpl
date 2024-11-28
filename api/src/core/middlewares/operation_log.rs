//! 操作日志中间件
//!
//! 此模块提供了一个操作日志中间件,用于记录系统中的所有操作行为。
//! 主要功能包括:
//! - 记录请求路径、方法、操作者、请求体和IP地址
//! - 异步处理日志记录,不影响主请求流程
//! - 根据请求路径和方法自动提取操作模块、动作和目标ID

use axum::{
    body::{to_bytes, Body},
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use database::repositories::OperationLogRepository;
use log::{error, info};
use std::net::SocketAddr;

use crate::{app_state::AppState, core::schema::Account};
use services::operation_log::{CreateLogParams, OperationLogService};

/// 操作日志中间件入口函数
///
/// # 参数
/// * `state` - 应用状态
/// * `request` - HTTP请求
/// * `next` - 下一个中间件
pub async fn operation_log(State(state): State<AppState>, request: Request<Body>, next: Next) -> Response {
    let (request_info, next_request) = extract_request_info(request).await;
    let response = next.run(next_request).await;

    spawn_log_task(state, request_info);
    response
}

/// 请求信息结构体
struct RequestInfo {
    path: String,
    method: String,
    operator: String,
    body: Option<String>,
    ip: String,
}

/// 提取请求信息
///
/// # 参数
/// * `request` - HTTP请求
///
/// # 返回
/// 返回提取的请求信息和重构的请求
async fn extract_request_info(request: Request<Body>) -> (RequestInfo, Request<Body>) {
    let path = request.uri().path().to_string();
    let method = request.method().to_string();
    let operator = extract_operator(&request);
    let ip = request
        .extensions()
        .get::<SocketAddr>()
        .map(|addr| addr.ip().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let (parts, body) = request.into_parts();
    let bytes = to_bytes(body, usize::MAX).await.unwrap_or_default();
    let body_str = String::from_utf8(bytes.to_vec()).ok();

    let next_request = Request::from_parts(parts.clone(), Body::from(bytes.clone()));

    (
        RequestInfo {
            path,
            method,
            operator,
            body: body_str,
            ip,
        },
        next_request,
    )
}

/// 从请求中提取操作者信息
///
/// # 参数
/// * `request` - HTTP请求
///
/// # 返回
/// 返回操作者账号,如果未找到则返回"anonymous"
fn extract_operator(request: &Request<Body>) -> String {
    request
        .extensions()
        .get::<Account>()
        .map(|account| account.0.clone())
        .unwrap_or_else(|| "anonymous".to_string())
}

/// 异步处理日志记录任务
///
/// # 参数
/// * `state` - 应用状态
/// * `request_info` - 请求信息
fn spawn_log_task(state: AppState, request_info: RequestInfo) {
    let method = request_info.method.clone();
    let path = request_info.path.clone();
    let operator = request_info.operator.clone();

    tokio::spawn(async move {
        if let Err(e) = create_operation_log(state, request_info).await {
            error!("Failed to create operation log: {}", e);
        } else {
            info!("Operation logged: {} {} by {}", method, path, operator);
        }
    });
}

/// 创建操作日志
///
/// # 参数
/// * `state` - 应用状态
/// * `request_info` - 请求信息
///
/// # 返回
/// 返回创建结果
async fn create_operation_log(state: AppState, request_info: RequestInfo) -> Result<(), String> {
    let service = OperationLogService::new(OperationLogRepository::new(state.db_state.db));
    let (module, action, target_id) = extract_operation_info(&request_info.path, &request_info.method);

    let params = CreateLogParams {
        operator: request_info.operator,
        module,
        action,
        target_id,
        description: format!("{} {}", request_info.method, request_info.path),
        request_path: request_info.path,
        request_method: request_info.method,
        request_body: request_info.body,
        ip_address: request_info.ip,
    };

    service.create_log(params).await.map_err(|e| e.to_string())
}

/// 从请求路径和方法中提取操作信息
///
/// # 参数
/// * `path` - 请求路径
/// * `method` - 请求方法
///
/// # 返回
/// 返回(模块,动作,目标ID)元组
fn extract_operation_info(path: &str, method: &str) -> (String, String, String) {
    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

    match parts.as_slice() {
        ["admins", id] => extract_admin_info(method, id),
        ["admins"] => extract_admins_info(method),
        ["roles", id] => extract_role_info(method, id),
        ["roles"] => extract_roles_info(method),
        _ => ("其他".to_string(), method.to_string(), "".to_string()),
    }
}

/// 提取管理员相关操作信息
fn extract_admin_info(method: &str, id: &str) -> (String, String, String) {
    (
        "管理员".to_string(),
        match method {
            "PUT" => "更新",
            "DELETE" => "删除",
            _ => method,
        }
        .to_string(),
        id.to_string(),
    )
}

/// 提取管理员列表相关操作信息
fn extract_admins_info(method: &str) -> (String, String, String) {
    (
        "管理员".to_string(),
        match method {
            "POST" => "创建",
            "GET" => "查询",
            _ => method,
        }
        .to_string(),
        "".to_string(),
    )
}

/// 提取角色相关操作信息
fn extract_role_info(method: &str, id: &str) -> (String, String, String) {
    (
        "角色".to_string(),
        match method {
            "PUT" => "更新",
            "DELETE" => "删除",
            _ => method,
        }
        .to_string(),
        id.to_string(),
    )
}

/// 提取角色列表相关操作信息
fn extract_roles_info(method: &str) -> (String, String, String) {
    (
        "角色".to_string(),
        match method {
            "POST" => "创建",
            "GET" => "查询",
            _ => method,
        }
        .to_string(),
        "".to_string(),
    )
}
