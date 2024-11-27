use entities::Admin;
use serde::{Deserialize, Serialize};
use services::admin::{CreateAdminParams, UpdateAdminParams, UpdateAdminRoleParams};
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

impl From<CreateAdminRequest> for CreateAdminParams {
    fn from(req: CreateAdminRequest) -> Self {
        Self {
            account: req.account,
            password: req.password,
            name: req.name,
            role_name: req.role_name,
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateAdminRequest {
    pub name: Option<String>,
    #[validate(length(min = 6, max = 32, message = "密码长度必须在6-32个字符之间"))]
    pub password: Option<String>,
    pub role_name: Option<String>,
}

impl UpdateAdminRequest {
    pub fn to_params(self, id: String) -> UpdateAdminParams {
        UpdateAdminParams {
            id,
            name: self.name,
            password: self.password,
            role_name: self.role_name,
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateAdminRoleRequest {
    #[validate(length(min = 2, max = 32, message = "角色名称长度必须在2-32个字符之间"))]
    pub role_name: String,
}

impl UpdateAdminRoleRequest {
    pub fn to_params(self, id: String) -> UpdateAdminRoleParams {
        UpdateAdminRoleParams {
            id,
            role_name: self.role_name,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AdminItem {
    pub id: String,
    pub account: String,
    pub name: String,
    pub role_name: String,
    pub created_at: u64,
}

impl From<Admin> for AdminItem {
    fn from(admin: Admin) -> Self {
        AdminItem {
            id: admin.base.id,
            account: admin.secret.account,
            name: admin.name,
            role_name: admin.role_name,
            created_at: admin.base.created_at,
        }
    }
}
