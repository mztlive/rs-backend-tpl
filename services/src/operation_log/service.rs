use super::repository::IOperationLogRepository;
use super::types::CreateLogParams;
use crate::errors::Result;
use entities::OperationLog;

pub struct OperationLogService<T: IOperationLogRepository> {
    repo: T,
}

impl<T: IOperationLogRepository> OperationLogService<T> {
    pub fn new(repo: T) -> Self {
        Self { repo }
    }

    pub async fn create_log(&self, params: CreateLogParams) -> Result<()> {
        let id = libs::next_id().await;

        let log = OperationLog::new(
            id,
            params.operator,
            params.module,
            params.action,
            params.target_id,
            params.description,
            params.request_path,
            params.request_method,
            params.request_body,
            params.ip_address,
        );

        self.repo.create(&log).await?;
        Ok(())
    }
}
