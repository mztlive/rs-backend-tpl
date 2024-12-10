//! Message entity module for handling system messages
//!
//! This module defines the message entity and its associated types,
//! supporting different message channels and statuses.

use std::fmt::Display;

use super::errors::{Error, Result};
use entity_core::BaseModel;
use entity_macros::Entity;
use serde::{Deserialize, Serialize};

const MAX_RETRY_TIMES: u8 = 10;

/// Message delivery channel enumeration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MessageChannel {
    /// Email
    Email,
    /// SMS
    SMS,
    /// WebSocket message
    WebSocket,
    /// Internal message
    InternalMessage,
}

impl Display for MessageChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Message status enumeration
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum MessageStatus {
    /// Pending
    Pending,
    /// Sent
    Sent,
    /// Failed
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

/// Message entity structure
#[derive(Debug, Serialize, Deserialize, Entity)]
pub struct Message {
    /// Base model fields including ID and timestamps
    #[serde(flatten)]
    pub base: BaseModel,

    /// Message delivery channel
    pub channel: MessageChannel,

    /// Message recipient (email/phone etc)
    pub recipient: String,

    /// Message subject
    pub subject: String,

    /// Message content
    pub content: String,

    /// Current message status
    pub status: MessageStatus,

    /// Error message if sending failed
    pub error: Option<String>,

    /// Number of retry attempts
    pub retry_times: u8,
}

impl Message {
    /// Creates a new message instance
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the message
    /// * `channel` - Channel to deliver the message through
    /// * `recipient` - Recipient of the message
    /// * `subject` - Message subject
    /// * `content` - Message content
    ///
    /// # Returns
    ///
    /// A new Message instance with Pending status
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
            retry_times: 0,
        }
    }

    pub fn add_retry_times(&mut self) -> crate::errors::Result<()> {
        if self.retry_times >= MAX_RETRY_TIMES {
            return Err(Error::LogicError("重试次数超过最大限制".to_string()));
        }

        self.retry_times = self.retry_times + 1;
        Ok(())
    }
}

impl MessageChannel {
    /// Converts a string to MessageChannel enumeration
    ///
    /// # Arguments
    ///
    /// * `s` - The string to convert
    ///
    /// # Returns
    ///
    /// Result<MessageChannel>, if conversion is successful, returns the corresponding enumeration value;
    /// if unsuccessful, returns Error::LogicError
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
