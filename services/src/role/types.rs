use entities::RouteItem;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoleParams {
    pub name: String,
    pub permissions: Vec<RouteItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRoleParams {
    pub id: String,
    pub name: Option<String>,
    pub permissions: Option<Vec<RouteItem>>,
} 