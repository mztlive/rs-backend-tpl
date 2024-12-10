use super::dto::*;
use super::repository::IBrandRepository;
use crate::errors::Result;
use entities::product::Brand;

pub struct BrandService<R>
where
    R: IBrandRepository,
{
    repo: R,
}

impl<R> BrandService<R>
where
    R: IBrandRepository,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn create_brand(&self, params: CreateBrandParams) -> Result<()> {
        let brand = Brand::new(
            libs::next_id().await,
            params.name,
            params.logo,
        );

        self.repo.create(&brand).await
    }

    pub async fn update_brand(&self, params: UpdateBrandParams) -> Result<()> {
        let mut brand = self
            .repo
            .find_by_id(&params.id)
            .await?
            .ok_or("品牌不存在")?;

        params.apply(&mut brand);

        self.repo.update(&brand).await
    }

    pub async fn get_brand_list(&self) -> Result<Vec<BrandDetail>> {
        let brands = self.repo.find_all().await?;
        let mut brand_list = brands
            .iter()
            .map(|b| BrandDetail {
                id: b.base.id.clone(),
                name: b.name.clone(),
                logo: b.logo.clone(),
                description: b.description.clone(),
                website: b.website.clone(),
                sort_order: b.sort_order,
                status: b.status.clone(),
                created_at: b.base.created_at,
            })
            .collect::<Vec<_>>();

        brand_list.sort_by_key(|b| b.sort_order);
        Ok(brand_list)
    }
} 