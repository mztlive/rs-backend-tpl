use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAdminParams {
    pub account: String,
    pub password: String,
    pub name: String,
    pub role_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAdminParams {
    pub id: String,
    pub name: Option<String>,
    pub password: Option<String>,
    pub role_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAdminRoleParams {
    pub id: String,
    pub role_name: String,
} 