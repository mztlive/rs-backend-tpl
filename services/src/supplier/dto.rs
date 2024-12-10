use serde::{Deserialize, Serialize};
use validator::Validate;
use entities::product::{Supplier, SupplierStatus};

#[derive(Debug, Deserialize, Validate)]
pub struct CreateSupplierParams {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub contact_person: String,
    #[validate(length(min = 1))]
    pub contact_phone: String,
    pub contact_email: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSupplierParams {
    pub id: String,
    pub name: Option<String>,
    pub contact_person: Option<String>,
    pub contact_phone: Option<String>,
    pub contact_email: Option<String>,
    pub address: Option<String>,
    pub status: Option<SupplierStatus>,
}

impl UpdateSupplierParams {
    pub fn apply(&self, supplier: &mut Supplier) {
        if let Some(name) = &self.name {
            supplier.name = name.clone();
        }
        if let Some(contact_person) = &self.contact_person {
            supplier.contact_person = contact_person.clone();
        }
        if let Some(contact_phone) = &self.contact_phone {
            supplier.contact_phone = contact_phone.clone();
        }
        if let Some(contact_email) = &self.contact_email {
            supplier.contact_email = Some(contact_email.clone());
        }
        if let Some(address) = &self.address {
            supplier.address = Some(address.clone());
        }
        if let Some(status) = &self.status {
            supplier.status = status.clone();
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SupplierDetail {
    pub id: String,
    pub name: String,
    pub contact_person: String,
    pub contact_phone: String,
    pub contact_email: Option<String>,
    pub address: Option<String>,
    pub status: SupplierStatus,
    pub created_at: u64,
} 