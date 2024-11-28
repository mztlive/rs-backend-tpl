use std::fmt::Display;

use super::errors::{Error, Result};
use entity_base::BaseModel;
use entity_derive::Entity;
use serde::{Deserialize, Serialize};

/// 消息发送渠道枚举
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MessageChannel {
    /// 电子邮件
    Email,
    /// 短信
    SMS,
    /// WebSocket消息
    WebSocket,
    /// 系统内部消息
    InternalMessage,
}

/// 消息发送状态枚举
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum MessageStatus {
    /// 等待发送
    Pending,
    /// 发送成功
    Sent,
    /// 发送失败
    Failed,
}

impl Display for MessageStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageStatus::Pending => write!(f, "Pending"),
            MessageStatus::Sent => write!(f, "Sent"),
            MessageStatus::Failed => write!(f, "Failed"),
        }
    }
}

/// 消息实体结构体
#[derive(Debug, Serialize, Deserialize, Entity)]
pub struct Message {
    /// 基础模型字段
    #[serde(flatten)]
    pub base: BaseModel,
    /// 消息发送渠道
    pub channel: MessageChannel,
    /// 接收者(邮箱/手机号等)
    pub recipient: String,
    /// 消息主题
    pub subject: String,
    /// 消息内容
    pub content: String,
    /// 发送状态
    pub status: MessageStatus,
    /// 错误信息(如果发送失败)
    pub error: Option<String>,
}

impl Message {
    /// 创建新的消息实例
    ///
    /// # 参数
    ///
    /// * `id` - 消息唯一标识符
    /// * `channel` - 消息发送渠道
    /// * `recipient` - 接收者信息
    /// * `subject` - 消息主题
    /// * `content` - 消息内容
    ///
    /// # 返回值
    ///
    /// 返回一个新的Message实例，初始状态为Pending
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
    /// 从字符串转换为MessageChannel枚举
    ///
    /// # 参数
    ///
    /// * `s` - 要转换的字符串
    ///
    /// # 返回值
    ///
    /// 返回Result<MessageChannel>，如果转换成功返回对应的枚举值，
    /// 如果失败返回Error::LogicError
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
