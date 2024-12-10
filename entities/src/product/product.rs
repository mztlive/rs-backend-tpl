use std::fmt::Display;

use super::sku::{PriceChangeType, SKU};
use crate::{errors::Result, Error};
use entity_core::BaseModel;
use entity_macros::Entity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Entity)]
pub struct Product {
    #[serde(flatten)]
    pub base: BaseModel,
    pub name: String,
    pub description: String,
    pub brand_id: String,
    pub category_id: String,
    pub supplier_id: String,
    pub main_image: String,
    pub images: Vec<String>,
    pub status: ProductStatus,
    pub sort_order: i32,
    pub is_featured: bool,
    #[serde(default)]
    pub skus: Vec<SKU>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ProductStatus {
    Draft,    // 草稿
    Active,   // 上架
    Inactive, // 下架
}

impl Display for ProductStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProductStatus::Draft => write!(f, "draft"),
            ProductStatus::Active => write!(f, "active"),
            ProductStatus::Inactive => write!(f, "inactive"),
        }
    }
}

impl Product {
    pub fn add_sku(&mut self, sku: SKU) {
        self.skus.push(sku);
    }

    pub fn remove_sku(&mut self, sku_id: &str) {
        self.skus.retain(|sku| sku.base.id != sku_id);
    }

    pub fn get_sku(&self, sku_id: &str) -> Option<&SKU> {
        self.skus.iter().find(|sku| sku.base.id == sku_id)
    }

    pub fn get_sku_mut(&mut self, sku_id: &str) -> Option<&mut SKU> {
        self.skus.iter_mut().find(|sku| sku.base.id == sku_id)
    }

    pub fn builder() -> ProductBuilder {
        ProductBuilder::new()
    }

    pub fn activate(&mut self) -> Result<()> {
        match self.status {
            ProductStatus::Draft | ProductStatus::Inactive => {
                self.status = ProductStatus::Active;
                for sku in &mut self.skus {
                    sku.activate()?;
                }
                Ok(())
            }
            ProductStatus::Active => Ok(()),
        }
    }

    pub fn deactivate(&mut self) -> Result<()> {
        match self.status {
            ProductStatus::Active => {
                self.status = ProductStatus::Inactive;
                for sku in &mut self.skus {
                    sku.deactivate()?;
                }
                Ok(())
            }
            ProductStatus::Inactive => Ok(()),
            ProductStatus::Draft => Err(Error::from_str("草稿状态的商品不能下架")),
        }
    }

    /// 调整所有 SKU 价格
    ///
    /// # Arguments
    /// * `change_type` - 调价方式
    pub fn change_price(&mut self, change_type: &PriceChangeType) -> Result<()> {
        // 验证商品状态
        if matches!(self.status, ProductStatus::Draft) {
            return Err(Error::from_str("草稿状态的商品不能调价"));
        }

        // 调整所有 SKU 的价格
        for sku in &mut self.skus {
            sku.change_price(change_type.clone())?;
        }

        Ok(())
    }
}

#[derive(Default)]
pub struct ProductBuilder {
    id: Option<String>,
    name: Option<String>,
    description: Option<String>,
    brand_id: Option<String>,
    category_id: Option<String>,
    supplier_id: Option<String>,
    main_image: Option<String>,
    images: Option<Vec<String>>,
    status: Option<ProductStatus>,
    sort_order: Option<i32>,
    is_featured: Option<bool>,
    skus: Vec<SKU>,
}

impl ProductBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn brand_id(mut self, brand_id: String) -> Self {
        self.brand_id = Some(brand_id);
        self
    }

    pub fn category_id(mut self, category_id: String) -> Self {
        self.category_id = Some(category_id);
        self
    }

    pub fn supplier_id(mut self, supplier_id: String) -> Self {
        self.supplier_id = Some(supplier_id);
        self
    }

    pub fn main_image(mut self, main_image: String) -> Self {
        self.main_image = Some(main_image);
        self
    }

    pub fn images(mut self, images: Vec<String>) -> Self {
        self.images = Some(images);
        self
    }

    pub fn status(mut self, status: ProductStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn sort_order(mut self, sort_order: i32) -> Self {
        self.sort_order = Some(sort_order);
        self
    }

    pub fn is_featured(mut self, is_featured: bool) -> Self {
        self.is_featured = Some(is_featured);
        self
    }

    pub fn add_sku(mut self, sku: SKU) -> Self {
        self.skus.push(sku);
        self
    }

    pub fn build(self) -> Result<Product> {
        let id = self.id.ok_or(Error::from_str("id is required"))?;
        let name = self.name.ok_or(Error::from_str("name is required"))?;
        let description = self
            .description
            .ok_or(Error::from_str("description is required"))?;

        let brand_id = self.brand_id.ok_or(Error::from_str("brand_id is required"))?;
        let category_id = self
            .category_id
            .ok_or(Error::from_str("category_id is required"))?;

        let supplier_id = self
            .supplier_id
            .ok_or(Error::from_str("supplier_id is required"))?;

        let main_image = self.main_image.ok_or(Error::from_str("main_image is required"))?;

        let images = self.images.unwrap_or_default();

        Ok(Product {
            base: BaseModel::new(id),
            name,
            description,
            brand_id,
            category_id,
            supplier_id,
            main_image,
            images,
            status: self.status.unwrap_or(ProductStatus::Draft),
            sort_order: self.sort_order.unwrap_or(0),
            is_featured: self.is_featured.unwrap_or(false),
            skus: self.skus,
        })
    }
}
