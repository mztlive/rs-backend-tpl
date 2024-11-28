use entities::OperationLog;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateLogParams {
    pub operator: String,
    pub module: String,
    pub action: String,
    pub target_id: String,
    pub description: String,
    pub request_path: String,
    pub request_method: String,
    pub request_body: Option<String>,
    pub ip_address: String,
}

impl CreateLogParams {
    pub async fn to_entity(&self) -> OperationLog {
        OperationLog::new(
            libs::next_id().await,
            &self.operator,
            &self.module,
            &self.action,
            &self.target_id,
            &self.description,
            &self.request_path,
            &self.request_method,
            self.request_body.as_deref(),
            &self.ip_address,
        )
    }
}
