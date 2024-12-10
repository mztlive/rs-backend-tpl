use serde::{Deserialize, Serialize};
use validator::Validate;
use entities::product::{Category, CategoryStatus};

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCategoryParams {
    #[validate(length(min = 1, max = 50))]
    pub name: String,
    pub parent_id: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCategoryParams {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub sort_order: Option<i32>,
    pub status: Option<CategoryStatus>,
}

impl UpdateCategoryParams {
    pub fn apply(&self, category: &mut Category) {
        if let Some(name) = &self.name {
            category.name = name.clone();
        }
        if let Some(description) = &self.description {
            category.description = Some(description.clone());
        }
        if let Some(image) = &self.image {
            category.image = Some(image.clone());
        }
        if let Some(sort_order) = self.sort_order {
            category.sort_order = sort_order;
        }
        if let Some(status) = &self.status {
            category.status = status.clone();
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CategoryDetail {
    pub id: String,
    pub name: String,
    pub parent_id: Option<String>,
    pub level: i32,
    pub path: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub sort_order: i32,
    pub status: CategoryStatus,
    pub created_at: u64,
} 