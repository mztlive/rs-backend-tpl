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
