use crate::errors::Result;
use database::repositories::{IRepository, OperationLogRepository};
use entities::OperationLog;
use mongodb::Database;

use super::types::CreateLogParams;

pub struct OperationLogService {
    repo: OperationLogRepository,
}

impl OperationLogService {
    pub fn new(database: Database) -> Self {
        Self {
            repo: OperationLogRepository::new(database),
        }
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
