use super::dto::*;
use super::repository::ISupplierRepository;
use crate::errors::Result;
use entities::product::Supplier;

pub struct SupplierService<R>
where
    R: ISupplierRepository,
{
    repo: R,
}

impl<R> SupplierService<R>
where
    R: ISupplierRepository,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn create_supplier(&self, params: CreateSupplierParams) -> Result<()> {
        let supplier = Supplier::new(
            libs::next_id().await,
            params.name,
            params.contact_person,
            params.contact_phone,
        );

        self.repo.create(&supplier).await
    }

    pub async fn update_supplier(&self, params: UpdateSupplierParams) -> Result<()> {
        let mut supplier = self
            .repo
            .find_by_id(&params.id)
            .await?
            .ok_or("供应商不存在")?;

        params.apply(&mut supplier);

        self.repo.update(&supplier).await
    }

    pub async fn get_supplier_list(&self) -> Result<Vec<SupplierDetail>> {
        let suppliers = self.repo.find_all().await?;
        let supplier_list = suppliers
            .iter()
            .map(|s| SupplierDetail {
                id: s.base.id.clone(),
                name: s.name.clone(),
                contact_person: s.contact_person.clone(),
                contact_phone: s.contact_phone.clone(),
                contact_email: s.contact_email.clone(),
                address: s.address.clone(),
                status: s.status.clone(),
                created_at: s.base.created_at,
            })
            .collect();

        Ok(supplier_list)
    }
} 