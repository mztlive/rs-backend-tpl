use entities::{Role, RouteItem};
use serde::{Deserialize, Serialize};
use services::role::{CreateRoleParams, UpdateRoleParams};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateRoleRequest {
    #[validate(length(min = 2, max = 32, message = "角色名称长度必须在2-32个字符之间"))]
    pub name: String,
    pub permissions: Vec<RouteItem>,
}

impl From<CreateRoleRequest> for CreateRoleParams {
    fn from(req: CreateRoleRequest) -> Self {
        Self {
            name: req.name,
            permissions: req.permissions,
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateRoleRequest {
    pub name: Option<String>,
    pub permissions: Option<Vec<RouteItem>>,
}

impl UpdateRoleRequest {
    pub fn to_params(self, id: String) -> UpdateRoleParams {
        UpdateRoleParams {
            id,
            name: self.name,
            permissions: self.permissions,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct RoleItem {
    pub id: String,
    pub name: String,
    pub permissions: Vec<RouteItem>,
    pub created_at: u64,
}

impl From<Role> for RoleItem {
    fn from(role: Role) -> Self {
        RoleItem {
            id: role.base.id,
            name: role.name,
            permissions: role.permissions,
            created_at: role.base.created_at,
        }
    }
} 