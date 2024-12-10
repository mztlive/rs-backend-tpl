/// 商品服务，提供商品和 SKU 相关的业务逻辑
///
/// # 功能
/// - 商品的创建、查询
/// - SKU 的创建、更新、删除
/// - 商品和 SKU 的关联管理
use super::dto::*;
use super::repository::IProductRepository;
use crate::brand::IBrandRepository;
use crate::category::ICategoryRepository;
use crate::errors::Result;
use crate::supplier::ISupplierRepository;
use entities::product::*;

/// 商品服务结构体
pub struct ProductService<R, B, C, SP>
where
    R: IProductRepository,
    B: IBrandRepository,
    C: ICategoryRepository,
    SP: ISupplierRepository,
{
    repo: R,
    brand_repo: B,
    category_repo: C,
    supplier_repo: SP,
}

impl<R, B, C, SP> ProductService<R, B, C, SP>
where
    R: IProductRepository,
    B: IBrandRepository,
    C: ICategoryRepository,
    SP: ISupplierRepository,
{
    /// 创建商品服务实例
    pub fn new(repo: R, brand_repo: B, category_repo: C, supplier_repo: SP) -> Self {
        Self {
            repo,
            brand_repo,
            category_repo,
            supplier_repo,
        }
    }

    /// 验证商品关联的品牌、分类、供应商是否存在, 在创建商品时调用
    ///
    /// # Arguments
    /// * `brand_id` - 品牌 ID
    /// * `category_id` - 分类 ID
    /// * `supplier_id` - 供应商 ID
    ///
    /// # Returns
    /// * `Ok(())` - 验证通过
    /// * `Err` - 验证失败，返回具体错误信息
    async fn validate_relations(&self, brand_id: &str, category_id: &str, supplier_id: &str) -> Result<()> {
        if self.brand_repo.find_by_id(brand_id).await?.is_none() {
            return Err("品牌不存在".into());
        }

        if self.category_repo.find_by_id(category_id).await?.is_none() {
            return Err("分类不存在".into());
        }

        if self.supplier_repo.find_by_id(supplier_id).await?.is_none() {
            return Err("供应商不存在".into());
        }

        Ok(())
    }

    /// 创建商品
    ///
    /// # Arguments
    /// * `params` - 创建商品的参数
    pub async fn create_product(&self, params: CreateProductParams) -> Result<()>
    where
        B: IBrandRepository,
        C: ICategoryRepository,
        SP: ISupplierRepository,
    {
        // 验证关联数据
        self.validate_relations(&params.brand_id, &params.category_id, &params.supplier_id)
            .await?;

        let product = Product::builder()
            .id(libs::next_id().await)
            .name(params.name)
            .description(params.description)
            .brand_id(params.brand_id)
            .category_id(params.category_id)
            .supplier_id(params.supplier_id)
            .main_image(params.main_image)
            .images(params.images)
            .is_featured(params.is_featured)
            .build()
            .map_err(|e| e.to_string())?;

        self.repo.create(&product).await
    }

    /// 获取商品详情
    ///
    /// # Arguments
    /// * `id` - 商品 ID
    ///
    /// # Returns
    /// * `Ok(Some(ProductDetail))` - 商品存在时返回商品详情
    /// * `Ok(None)` - 商品不存在
    pub async fn get_product_detail(&self, id: &str) -> Result<Option<ProductDetail>> {
        let product = match self.repo.find_by_id(id).await? {
            Some(p) => p,
            None => return Ok(None),
        };

        Ok(Some(ProductDetail::from(product)))
    }

    /// 创建 SKU
    ///
    /// # Arguments
    /// * `params` - 创建 SKU 的参数
    pub async fn create_sku(&self, params: CreateSkuParams) -> Result<()> {
        let mut product = self
            .repo
            .find_by_id(&params.product_id)
            .await?
            .ok_or("商品不存在")?;

        let sku = SKU::new(
            libs::next_id().await,
            params.sku_code,
            params.name,
            params.price,
            params.stock,
            params.specs,
        );

        product.add_sku(sku);
        self.repo.update(&product).await
    }

    /// 更新 SKU
    ///
    /// # Arguments
    /// * `params` - 更新 SKU 的参数
    pub async fn update_sku(&self, params: UpdateSkuParams) -> Result<()> {
        let mut product = self.repo.find_by_id(&params.id).await?.ok_or("商品不存在")?;
        let sku = product.get_sku_mut(&params.id).ok_or("SKU不存在")?;

        params.apply(sku);
        self.repo.update(&product).await
    }

    /// 删除 SKU
    ///
    /// # Arguments
    /// * `sku_id` - SKU ID
    pub async fn delete_sku(&self, sku_id: &str) -> Result<()> {
        let mut product = self.repo.find_by_id(&sku_id).await?.ok_or("商品不存在")?;

        product.remove_sku(sku_id);
        self.repo.update(&product).await
    }

    /// 创建商品及其 SKUs
    ///
    /// # Arguments
    /// * `params` - 创建商品和 SKUs 的参数
    pub async fn create_product_with_skus(&self, params: CreateProductWithSkusParams) -> Result<()> {
        // 验证关联数据
        self.validate_relations(&params.brand_id, &params.category_id, &params.supplier_id)
            .await?;

        let mut builder = Product::builder()
            .id(libs::next_id().await)
            .name(params.name)
            .description(params.description)
            .brand_id(params.brand_id)
            .category_id(params.category_id)
            .supplier_id(params.supplier_id)
            .main_image(params.main_image)
            .images(params.images)
            .is_featured(params.is_featured);

        // 添加 SKUs
        for sku_params in params.skus {
            let sku = SKU::new(
                libs::next_id().await,
                sku_params.sku_code,
                sku_params.name,
                sku_params.price,
                sku_params.stock,
                sku_params.specs,
            );
            builder = builder.add_sku(sku);
        }

        let product = builder.build().map_err(|e| e.to_string())?;
        self.repo.create(&product).await
    }

    /// 更新商品信息
    ///
    /// # Arguments
    /// * `params` - 更新商品的参数
    pub async fn update_product(&self, params: UpdateProductParams) -> Result<()> {
        let mut product = self.repo.find_by_id(&params.id).await?.ok_or("商品不存在")?;

        // 验证关联数据
        if let Some(brand_id) = &params.brand_id {
            if self.brand_repo.find_by_id(brand_id).await?.is_none() {
                return Err("品牌不存在".into());
            }
        }

        if let Some(category_id) = &params.category_id {
            if self.category_repo.find_by_id(category_id).await?.is_none() {
                return Err("分类不存在".into());
            }
        }

        if let Some(supplier_id) = &params.supplier_id {
            if self.supplier_repo.find_by_id(supplier_id).await?.is_none() {
                return Err("供应商不存在".into());
            }
        }

        params.apply(&mut product);
        self.repo.update(&product).await
    }

    /// 批量上架商品
    ///
    /// # Arguments
    /// * `product_ids` - 要上架的商品 ID 列表
    ///
    /// # Returns
    /// * `Ok(())` - 操作成功
    /// * `Err` - 操作失败，返回具体错误信息
    pub async fn batch_activate_products(&self, product_ids: &[String]) -> Result<()> {
        let mut products = Vec::new();
        for id in product_ids {
            let mut product = self.repo.find_by_id(id).await?.ok_or("商品不存在")?;
            product.activate()?;
            products.push(product);
        }
        self.repo.update_many(&products).await
    }

    /// 批量下架商品
    ///
    /// # Arguments
    /// * `product_ids` - 要下架的商��� ID 列表
    pub async fn batch_deactivate_products(&self, product_ids: &[String]) -> Result<()> {
        let mut products = Vec::new();
        for id in product_ids {
            let mut product = self.repo.find_by_id(id).await?.ok_or("商品不存在")?;
            product.deactivate()?;
            products.push(product);
        }
        self.repo.update_many(&products).await
    }

    /// 批量调整商品价格
    ///
    /// # Arguments
    /// * `product_ids` - 商品 ID 列表
    /// * `change_type` - 调价方式
    pub async fn batch_change_products_price(
        &self,
        product_ids: &[String],
        change_type: PriceChangeType,
    ) -> Result<()> {
        let mut products = Vec::new();
        for id in product_ids {
            let mut product = self.repo.find_by_id(id).await?.ok_or("商品不存在")?;
            product.change_price(&change_type)?;
            products.push(product);
        }
        self.repo.update_many(&products).await
    }
}
