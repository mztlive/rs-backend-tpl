use crate::errors::Result;
use async_trait::async_trait;
use entities::OperationLog;

#[async_trait]
pub trait IOperationLogRepository: Send + Sync {
    async fn create(&self, log: &OperationLog) -> Result<()>;
}
