use mongodb::Database;

use super::{collection_names::OPERATION_LOG, IRepository};
use entities::OperationLog;

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
