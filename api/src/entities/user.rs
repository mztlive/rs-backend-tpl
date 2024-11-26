use entity_base::BaseModel;
use entity_derive::Entity;
use serde::{Deserialize, Serialize};

use crate::rbac::RBACUser;

use super::common::Secret;

#[derive(Debug, Serialize, Deserialize, Default, Clone, Entity)]
#[serde(default)]
pub struct User {
    #[serde(flatten)]
    pub base: BaseModel,
    pub secret: Secret,
    pub name: String,
    pub age: u8,
    pub avatar: String,
    pub is_active: bool,
    pub role_name: String,
}

impl RBACUser for User {
    fn account(&self) -> String {
        self.name.clone()
    }

    fn role_name(&self) -> String {
        self.role_name.clone()
    }
}

impl User {
    pub fn new(id: String, secret: Secret, name: String, role_name: String) -> Self {
        Self {
            base: BaseModel::new(id),
            secret,
            name,
            age: 0,
            avatar: String::new(),
            is_active: true,
            role_name,
        }
    }
}
