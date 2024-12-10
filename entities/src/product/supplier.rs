use entity_core::BaseModel;
use entity_macros::Entity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Entity)]
pub struct Supplier {
    #[serde(flatten)]
    pub base: BaseModel,
    pub name: String,
    pub contact_person: String,
    pub contact_phone: String,
    pub contact_email: Option<String>,
    pub address: Option<String>,
    pub status: SupplierStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SupplierStatus {
    Active,
    Inactive,
}

impl Supplier {
    pub fn new(
        id: String,
        name: String,
        contact_person: String,
        contact_phone: String,
    ) -> Self {
        Self {
            base: BaseModel::new(id),
            name,
            contact_person,
            contact_phone,
            contact_email: None,
            address: None,
            status: SupplierStatus::Active,
        }
    }
} 