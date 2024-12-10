use entities::product::{
    Brand, BrandStatus, Category, CategoryStatus, Product, ProductStatus, SkuSpec, SkuStatus, Supplier,
    SupplierStatus, SKU,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

// Product DTOs
#[derive(Debug, Deserialize, Validate)]
pub struct CreateProductParams {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub description: String,
    pub brand_id: String,
    pub category_id: String,
    pub supplier_id: String,
    pub main_image: String,
    pub images: Vec<String>,
    pub is_featured: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProductParams {
    pub id: String,
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
}

impl UpdateProductParams {
    pub fn apply(&self, product: &mut Product) {
        if let Some(name) = &self.name {
            product.name = name.clone();
        }
        if let Some(description) = &self.description {
            product.description = description.clone();
        }
        if let Some(brand_id) = &self.brand_id {
            product.brand_id = brand_id.clone();
        }
        if let Some(category_id) = &self.category_id {
            product.category_id = category_id.clone();
        }
        if let Some(supplier_id) = &self.supplier_id {
            product.supplier_id = supplier_id.clone();
        }
        if let Some(main_image) = &self.main_image {
            product.main_image = main_image.clone();
        }
        if let Some(images) = &self.images {
            product.images = images.clone();
        }
        if let Some(status) = &self.status {
            product.status = status.clone();
        }
        if let Some(sort_order) = self.sort_order {
            product.sort_order = sort_order;
        }
        if let Some(is_featured) = self.is_featured {
            product.is_featured = is_featured;
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ProductDetail {
    pub id: String,
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
    pub created_at: u64,
    pub skus: Vec<SkuDetail>,
}

impl From<Product> for ProductDetail {
    fn from(product: Product) -> Self {
        Self {
            id: product.base.id,
            name: product.name,
            description: product.description,
            brand_id: product.brand_id,
            category_id: product.category_id,
            supplier_id: product.supplier_id,
            main_image: product.main_image,
            images: product.images,
            status: product.status,
            sort_order: product.sort_order,
            is_featured: product.is_featured,
            created_at: product.base.created_at,
            skus: product.skus.into_iter().map(SkuDetail::from).collect(),
        }
    }
}

// SKU DTOs
#[derive(Debug, Deserialize, Validate)]
pub struct CreateSkuParams {
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

#[derive(Debug, Deserialize)]
pub struct UpdateSkuParams {
    pub id: String,
    pub name: Option<String>,
    pub price: Option<f64>,
    pub original_price: Option<f64>,
    pub stock: Option<i32>,
    pub specs: Option<Vec<SkuSpec>>,
    pub image: Option<String>,
    pub status: Option<SkuStatus>,
}

impl UpdateSkuParams {
    pub fn apply(&self, sku: &mut SKU) {
        if let Some(name) = &self.name {
            sku.name = name.clone();
        }

        if let Some(price) = self.price {
            sku.price = price;
        }

        if let Some(original_price) = self.original_price {
            sku.original_price = Some(original_price);
        }

        if let Some(stock) = self.stock {
            sku.stock = stock;
        }

        if let Some(specs) = &self.specs {
            sku.specs = specs.to_vec();
        }

        if let Some(image) = &self.image {
            sku.image = Some(image.clone());
        }

        if let Some(status) = &self.status {
            sku.status = status.clone();
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SkuDetail {
    pub id: String,
    pub sku_code: String,
    pub name: String,
    pub price: f64,
    pub original_price: Option<f64>,
    pub stock: i32,
    pub specs: Vec<SkuSpec>,
    pub image: Option<String>,
    pub status: SkuStatus,
    pub created_at: u64,
}

impl From<SKU> for SkuDetail {
    fn from(sku: SKU) -> Self {
        Self {
            id: sku.base.id,
            sku_code: sku.sku_code,
            name: sku.name,
            price: sku.price,
            original_price: sku.original_price,
            stock: sku.stock,
            specs: sku.specs,
            image: sku.image,
            status: sku.status,
            created_at: sku.base.created_at,
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProductWithSkusParams {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub description: String,
    pub brand_id: String,
    pub category_id: String,
    pub supplier_id: String,
    pub main_image: String,
    pub images: Vec<String>,
    pub is_featured: bool,
    pub skus: Vec<CreateSkuParams>,
}

#[derive(Debug, Deserialize)]
pub struct ProductSearchParams {
    pub keyword: Option<String>,         // 关键词搜索(名称、描述)
    pub category_id: Option<String>,     // 分类筛选
    pub brand_id: Option<String>,        // 品牌筛选
    pub supplier_id: Option<String>,     // 供应商筛选
    pub status: Option<ProductStatus>,   // 状态筛选
    pub is_featured: Option<bool>,       // 是否推荐
    pub price_range: Option<PriceRange>, // 价格区间
    pub sort_by: Option<ProductSortBy>,  // 排序方式
    pub page: Option<u32>,               // 分页
    pub page_size: Option<u32>,          // 每页数量
}

#[derive(Debug, Deserialize)]
pub struct PriceRange {
    pub min: Option<f64>,
    pub max: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub enum ProductSortBy {
    CreateTimeDesc, // 创建时间降序
    CreateTimeAsc,  // 创建时间升序
    PriceDesc,      // 价格降序
    PriceAsc,       // 价格升序
    SalesDesc,      // 销量降序
    SortOrderAsc,   // 自定义排序升序
}
