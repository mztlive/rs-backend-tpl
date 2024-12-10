use serde::{Deserialize, Serialize};
use validator::Validate;
use entities::product::{Brand, BrandStatus};

#[derive(Debug, Deserialize, Validate)]
pub struct CreateBrandParams {
    #[validate(length(min = 1, max = 50))]
    pub name: String,
    pub logo: String,
    pub description: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBrandParams {
    pub id: String,
    pub name: Option<String>,
    pub logo: Option<String>,
    pub description: Option<String>,
    pub website: Option<String>,
    pub sort_order: Option<i32>,
    pub status: Option<BrandStatus>,
}

impl UpdateBrandParams {
    pub fn apply(&self, brand: &mut Brand) {
        if let Some(name) = &self.name {
            brand.name = name.clone();
        }
        if let Some(logo) = &self.logo {
            brand.logo = logo.clone();
        }
        if let Some(description) = &self.description {
            brand.description = Some(description.clone());
        }
        if let Some(website) = &self.website {
            brand.website = Some(website.clone());
        }
        if let Some(sort_order) = self.sort_order {
            brand.sort_order = sort_order;
        }
        if let Some(status) = &self.status {
            brand.status = status.clone();
        }
    }
}

#[derive(Debug, Serialize)]
pub struct BrandDetail {
    pub id: String,
    pub name: String,
    pub logo: String,
    pub description: Option<String>,
    pub website: Option<String>,
    pub sort_order: i32,
    pub status: BrandStatus,
    pub created_at: u64,
} 