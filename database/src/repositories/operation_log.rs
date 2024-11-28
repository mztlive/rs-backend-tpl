use async_trait::async_trait;
use mongodb::Database;

use super::{collection_names::OPERATION_LOG, IRepository};
use entities::OperationLog;
use services::errors::Result as ServiceResult;
use services::operation_log::IOperationLogRepository;

pub struct OperationLogRepository {
    pub coll_name: String,
    database: Database,
}

impl OperationLogRepository {
    pub fn new(database: Database) -> Self {
        Self {
            coll_name: OPERATION_LOG.to_string(),
            database,
        }
    }
}

impl IRepository<OperationLog> for OperationLogRepository {
    fn get_collection_name(&self) -> &str {
        &self.coll_name
    }

    fn get_database(&self) -> &Database {
        &self.database
    }
}

#[async_trait]
impl IOperationLogRepository for OperationLogRepository {
    async fn create(&self, log: &OperationLog) -> ServiceResult<()> {
        IRepository::create(self, log).await?;
        Ok(())
    }
}
