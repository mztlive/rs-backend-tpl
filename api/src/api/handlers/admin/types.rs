use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateAdminRequest {
    #[validate(length(min = 3, max = 32, message = "账号长度必须在3-32个字符之间"))]
    pub account: String,
    #[validate(length(min = 6, max = 32, message = "密码长度必须在6-32个字符之间"))]
    pub password: String,
    pub name: String,
    pub role_name: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateAdminRequest {
    pub name: Option<String>,
    #[validate(length(min = 6, max = 32, message = "密码长度必须在6-32个字符之间"))]
    pub password: Option<String>,
    pub role_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AdminResponse {
    pub id: String,
    pub account: String,
    pub name: String,
    pub role_name: String,
    pub created_at: u64,
} 