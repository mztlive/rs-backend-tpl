use entity_core::BaseModel;
use entity_macros::Entity;
use serde::{Deserialize, Serialize};

/// 内部消息状态枚举
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum InternalMessageStatus {
    /// 未读状态
    Unread,
    /// 已读状态
    Read,
}

/// 内部消息实体结构体
#[derive(Debug, Serialize, Deserialize, Entity)]
pub struct InternalMessage {
    /// 基础模型字段
    #[serde(flatten)]
    pub base: BaseModel,
    /// 接收者ID
    pub recipient: String,
    /// 消息主题
    pub subject: String,
    /// 消息内容
    pub content: String,
    /// 消息读取状态
    pub status: InternalMessageStatus,
}

impl InternalMessage {
    /// 创建新的内部消息实例
    ///
    /// # 参数
    ///
    /// * `id` - 消息唯一标识符
    /// * `recipient` - 接收者ID
    /// * `subject` - 消息主题
    /// * `content` - 消息内容
    ///
    /// # 返回值
    ///
    /// 返回一个新的InternalMessage实例，初始状态为未读
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
