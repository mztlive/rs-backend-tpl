use entity_core::BaseModel;
use entity_macros::Entity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Entity)]
pub struct Category {
    #[serde(flatten)]
    pub base: BaseModel,
    pub name: String,
    pub parent_id: Option<String>,
    pub level: i32,
    pub path: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub sort_order: i32,
    pub status: CategoryStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CategoryStatus {
    Active,
    Inactive,
}

impl Category {
    pub fn new(id: String, name: String, parent_id: Option<String>, level: i32, path: String) -> Self {
        Self {
            base: BaseModel::new(id),
            name,
            parent_id,
            level,
            path,
            description: None,
            image: None,
            sort_order: 0,
            status: CategoryStatus::Active,
        }
    }
}
