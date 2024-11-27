use entity_base::BaseModel;
use entity_derive::Entity;
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
        operator: String,
        module: String,
        action: String,
        target_id: String,
        description: String,
        request_path: String,
        request_method: String,
        request_body: Option<String>,
        ip_address: String,
    ) -> Self {
        Self {
            base: BaseModel::new(id),
            operator,
            module,
            action,
            target_id,
            description,
            request_path,
            request_method,
            request_body,
            ip_address,
        }
    }
}
