use crate::errors::Error;
use crate::repositories::base::cursor_to_vec;
use async_trait::async_trait;
use entities::product::{Product, SKU};
use mongodb::{bson::doc, Database};
use services::errors::Result as ServiceResult;
use services::product::{IProductRepository, ProductDetail, ProductSearchParams, ProductSortBy};

use super::super::base::{IFilter, IPaginator};
use super::super::collection_names::{PRODUCT, SKU as SKU_COLL};
use super::super::IRepository;

pub struct ProductRepository {
    pub coll_name: String,
    pub sku_coll_name: String,
    database: Database,
}

impl ProductRepository {
    pub fn new(database: Database) -> Self {
        Self {
            coll_name: PRODUCT.to_string(),
            sku_coll_name: SKU_COLL.to_string(),
            database,
        }
    }
}

impl IFilter for ProductSearchParams {
    fn to_doc(&self) -> mongodb::bson::Document {
        let mut filter = doc! { "deleted_at": 0 };

        // 关键词搜索
        if let Some(keyword) = &self.keyword {
            filter.insert(
                "$or",
                doc! {
                    "name": { "$regex": keyword, "$options": "i" },
                    "description": { "$regex": keyword, "$options": "i" }
                },
            );
        }

        // 分类筛选
        if let Some(category_id) = &self.category_id {
            filter.insert("category_id", category_id);
        }

        // 品牌筛选
        if let Some(brand_id) = &self.brand_id {
            filter.insert("brand_id", brand_id);
        }

        // 供应商筛选
        if let Some(supplier_id) = &self.supplier_id {
            filter.insert("supplier_id", supplier_id);
        }

        // 状态筛选
        if let Some(status) = &self.status {
            filter.insert("status", status.to_string());
        }

        // 是否推荐
        if let Some(is_featured) = self.is_featured {
            filter.insert("is_featured", is_featured);
        }

        // 价格区间
        if let Some(price_range) = &self.price_range {
            let mut price_query = doc! {};
            if let Some(min) = price_range.min {
                price_query.insert("$gte", min);
            }
            if let Some(max) = price_range.max {
                price_query.insert("$lte", max);
            }
            if !price_query.is_empty() {
                filter.insert("skus.price", price_query);
            }
        }

        filter
    }
}

impl IPaginator for ProductSearchParams {
    fn skip(&self) -> u64 {
        ((self.page.unwrap_or(1).max(1) - 1) * self.page_size.unwrap_or(20)) as u64
    }

    fn limit(&self) -> i64 {
        self.page_size.unwrap_or(20) as i64
    }
}

impl IRepository<Product> for ProductRepository {
    fn get_collection_name(&self) -> &str {
        &self.coll_name
    }

    fn get_database(&self) -> &Database {
        &self.database
    }
}

#[async_trait]
impl IProductRepository for ProductRepository {
    async fn create(&self, product: &Product) -> ServiceResult<()> {
        Ok(IRepository::create(self, product).await?)
    }

    async fn update(&self, product: &Product) -> ServiceResult<()> {
        Ok(IRepository::update(self, product).await?)
    }

    async fn find_by_id(&self, id: &str) -> ServiceResult<Option<Product>> {
        Ok(IRepository::find_by_id(self, id).await?)
    }

    async fn find_all(&self) -> ServiceResult<Vec<Product>> {
        Ok(IRepository::find_all(self).await?)
    }

    async fn find_by_category(&self, category_id: &str) -> ServiceResult<Vec<Product>> {
        let cursor = self
            .get_database()
            .collection::<Product>(self.get_collection_name())
            .find(doc! {
                "category_id": category_id,
                "deleted_at": 0
            })
            .await
            .map_err(|e| Error::DatabaseError(e))?;

        Ok(cursor_to_vec(cursor).await?)
    }

    async fn find_featured(&self) -> ServiceResult<Vec<Product>> {
        let cursor = self
            .get_database()
            .collection::<Product>(self.get_collection_name())
            .find(doc! {
                "is_featured": true,
                "deleted_at": 0
            })
            .await
            .map_err(|e| Error::DatabaseError(e))?;

        Ok(cursor_to_vec(cursor).await?)
    }

    async fn search(&self, params: &ProductSearchParams) -> ServiceResult<(u64, Vec<ProductDetail>)> {
        let res = IRepository::search(self, params).await?;
        Ok((
            res.total as u64,
            res.items.into_iter().map(ProductDetail::from).collect(),
        ))
    }

    async fn update_many(&self, products: &[Product]) -> ServiceResult<()> {
        for product in products {
            self.get_database()
                .collection::<Product>(&self.get_collection_name())
                .update_one(
                    doc! {
                        "id": &product.base.id,
                        "deleted_at": 0
                    },
                    doc! { "$set": mongodb::bson::to_document(product).unwrap() },
                )
                .await
                .map_err(|e| Error::DatabaseError(e))?;
        }

        Ok(())
    }
}
