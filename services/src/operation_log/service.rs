use super::repository::IOperationLogRepository;
use super::dto::CreateLogParams;
use crate::errors::Result;

pub struct OperationLogService<T: IOperationLogRepository> {
    repo: T,
}

impl<T: IOperationLogRepository> OperationLogService<T> {
    pub fn new(repo: T) -> Self {
        Self { repo }
    }

    pub async fn create_log(&self, params: CreateLogParams) -> Result<()> {
        self.repo.create(&params.to_entity().await).await?;
        Ok(())
    }
}
