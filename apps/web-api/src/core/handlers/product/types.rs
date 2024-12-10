use serde::{Deserialize, Serialize};
use validator::Validate;
use entities::product::{
    Product, SKU, Category, Brand, Supplier,
    ProductStatus, SkuStatus, CategoryStatus, BrandStatus, SupplierStatus,
};

// Product DTOs
#[derive(Debug, Deserialize, Validate)]
pub struct CreateProductRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub description: String,
    pub brand_id: String,
    pub category_id: String,
    pub supplier_id: String,
    pub main_image: String,
    pub images: Vec<String>,
    pub is_featured: bool,
    pub seo_title: Option<String>,
    pub seo_keywords: Option<String>,
    pub seo_description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProductRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub brand_id: Option<String>,
    pub category_id: Option<String>,
    pub supplier_id: Option<String>,
    pub main_image: Option<String>,
    pub images: Option<Vec<String>>,
    pub status: Option<ProductStatus>,
    pub sort_order: Option<i32>,
    pub is_featured: Option<bool>,
    pub seo_title: Option<String>,
    pub seo_keywords: Option<String>,
    pub seo_description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProductResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub brand_id: String,
    pub category_id: String,
    pub supplier_id: String,
    pub main_image: String,
    pub images: Vec<String>,
    pub status: String,
    pub sort_order: i32,
    pub is_featured: bool,
    pub seo_title: Option<String>,
    pub seo_keywords: Option<String>,
    pub seo_description: Option<String>,
    pub created_at: u64,
    pub skus: Vec<SkuResponse>,
}

// SKU DTOs
#[derive(Debug, Deserialize, Validate)]
pub struct CreateSkuRequest {
    pub product_id: String,
    #[validate(length(min = 1))]
    pub sku_code: String,
    pub name: String,
    pub price: f64,
    pub original_price: Option<f64>,
    pub stock: i32,
    pub specs: Vec<SkuSpec>,
    pub image: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SkuResponse {
    pub id: String,
    pub product_id: String,
    pub sku_code: String,
    pub name: String,
    pub price: f64,
    pub original_price: Option<f64>,
    pub stock: i32,
    pub specs: Vec<SkuSpec>,
    pub image: Option<String>,
    pub status: String,
    pub created_at: u64,
}

// Category DTOs
#[derive(Debug, Deserialize, Validate)]
pub struct CreateCategoryRequest {
    #[validate(length(min = 1, max = 50))]
    pub name: String,
    pub parent_id: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CategoryResponse {
    pub id: String,
    pub name: String,
    pub parent_id: Option<String>,
    pub level: i32,
    pub path: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub sort_order: i32,
    pub status: String,
    pub created_at: u64,
}

// Brand DTOs
#[derive(Debug, Deserialize, Validate)]
pub struct CreateBrandRequest {
    #[validate(length(min = 1, max = 50))]
    pub name: String,
    pub logo: String,
    pub description: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct BrandResponse {
    pub id: String,
    pub name: String,
    pub logo: String,
    pub description: Option<String>,
    pub website: Option<String>,
    pub sort_order: i32,
    pub status: String,
    pub created_at: u64,
}

// Supplier DTOs
#[derive(Debug, Deserialize, Validate)]
pub struct CreateSupplierRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub contact_person: String,
    #[validate(length(min = 1))]
    pub contact_phone: String,
    pub contact_email: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SupplierResponse {
    pub id: String,
    pub name: String,
    pub contact_person: String,
    pub contact_phone: String,
    pub contact_email: Option<String>,
    pub address: Option<String>,
    pub status: String,
    pub created_at: u64,
}