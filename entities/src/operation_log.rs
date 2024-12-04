use entity_core::BaseModel;
use entity_macros::Entity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Entity)]
pub struct OperationLog {
    #[serde(flatten)]
    pub base: BaseModel,
    pub operator: String,             // 操作者账号
    pub module: String,               // 操作模块
    pub action: String,               // 操作动作
    pub target_id: String,            // 操作目标ID
    pub description: String,          // 操作描述
    pub request_path: String,         // 请求路径
    pub request_method: String,       // 请求方法
    pub request_body: Option<String>, // 请求体
    pub ip_address: String,           // IP地址
}

impl OperationLog {
    pub fn new(
        id: String,
        operator: &str,
        module: &str,
        action: &str,
        target_id: &str,
        description: &str,
        request_path: &str,
        request_method: &str,
        request_body: Option<&str>,
        ip_address: &str,
    ) -> Self {
        Self {
            base: BaseModel::new(id),
            operator: operator.to_string(),
            module: module.to_string(),
            action: action.to_string(),
            target_id: target_id.to_string(),
            description: description.to_string(),
            request_path: request_path.to_string(),
            request_method: request_method.to_string(),
            request_body: request_body.map(|s| s.to_string()),
            ip_address: ip_address.to_string(),
        }
    }
}
