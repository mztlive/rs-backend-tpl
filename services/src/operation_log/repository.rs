use async_trait::async_trait;
use entities::OperationLog;
use crate::errors::Result;

#[async_trait]
pub trait IOperationLogRepository: Send + Sync {
    async fn create(&self, log: &OperationLog) -> Result<()>;
} 