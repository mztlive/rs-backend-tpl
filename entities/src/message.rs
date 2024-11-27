use super::errors::{Error, Result};
use entity_base::BaseModel;
use entity_derive::Entity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MessageChannel {
    Email,
    SMS,
    WebSocket,
    InternalMessage,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum MessageStatus {
    Pending,
    Sent,
    Failed,
}

#[derive(Debug, Serialize, Deserialize, Entity)]
pub struct Message {
    #[serde(flatten)]
    pub base: BaseModel,
    pub channel: MessageChannel,
    pub recipient: String,     // 接收者(邮箱/手机号等)
    pub subject: String,       // 主题
    pub content: String,       // 内容
    pub status: MessageStatus, // 发送状态
    pub error: Option<String>, // 错误信息
}

impl Message {
    pub fn new(
        id: String,
        channel: MessageChannel,
        recipient: String,
        subject: String,
        content: String,
    ) -> Self {
        Self {
            base: BaseModel::new(id),
            channel,
            recipient,
            subject,
            content,
            status: MessageStatus::Pending,
            error: None,
        }
    }
}

impl MessageChannel {
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_uppercase().as_str() {
            "EMAIL" => Ok(Self::Email),
            "SMS" => Ok(Self::SMS),
            "WEBSOCKET" => Ok(Self::WebSocket),
            "INTERNAL" => Ok(Self::InternalMessage),
            _ => Err(Error::LogicError(format!("Invalid message channel: {}", s))),
        }
    }
}
