use serde::{Deserialize, Serialize};

/// 认证请求结构体
///
/// 用于接收客户端发送的登录认证请求
///
/// # 字段
///
/// * `account` - 用户账号
/// * `password` - 用户密码
#[derive(Deserialize)]
pub struct AuthRequest {
    pub account: String,
    pub password: String,
}

/// 认证响应结构体
///
/// 用于向客户端返回认证结果
///
/// # 字段
///
/// * `token` - JWT认证令牌
#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
}

/// 用户信息结构体
///
/// 用于向客户端返回用户基本信息
///
/// # 字段
///
/// * `user_id` - 用户ID(序列化时重命名为userid以配合antd pro)
/// * `name` - 用户名称
/// * `avatar` - 用户头像URL
#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    #[serde(rename = "userid")] // 为了配合antd pro
    pub user_id: String,
    pub name: String,
    pub avatar: String,
}
