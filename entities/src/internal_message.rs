use entity_base::BaseModel;
use entity_derive::Entity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum InternalMessageStatus {
    Unread,
    Read,
}

#[derive(Debug, Serialize, Deserialize, Entity)]
pub struct InternalMessage {
    #[serde(flatten)]
    pub base: BaseModel,
    pub recipient: String,             // 接收者ID
    pub subject: String,               // 主题
    pub content: String,               // 内容
    pub status: InternalMessageStatus, // 读取状态
}

impl InternalMessage {
    pub fn new(id: String, recipient: String, subject: String, content: String) -> Self {
        Self {
            base: BaseModel::new(id),
            recipient,
            subject,
            content,
            status: InternalMessageStatus::Unread,
        }
    }
}
