use entity_core::BaseModel;
use entity_macros::Entity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Entity)]
pub struct Brand {
    #[serde(flatten)]
    pub base: BaseModel,
    pub name: String,
    pub logo: String,
    pub description: Option<String>,
    pub website: Option<String>,
    pub sort_order: i32,
    pub status: BrandStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum BrandStatus {
    Active,
    Inactive,
}

impl Brand {
    pub fn new(id: String, name: String, logo: String) -> Self {
        Self {
            base: BaseModel::new(id),
            name,
            logo,
            description: None,
            website: None,
            sort_order: 0,
            status: BrandStatus::Active,
        }
    }
}
