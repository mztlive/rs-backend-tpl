use entities::errors::Error as EntityError;
use entities::MessageChannel;
use serde::{Deserialize, Serialize};
use services::message::{MessageQuery, SendMessageParams};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct SendMessageRequest {
    #[validate(length(min = 1))]
    pub recipient: String,
    #[validate(length(min = 1))]
    pub subject: String,
    #[validate(length(min = 1))]
    pub content: String,
    pub channel: String,
}

impl SendMessageRequest {
    pub fn into_params(self) -> Result<SendMessageParams, EntityError> {
        Ok(SendMessageParams {
            channel: MessageChannel::from_str(&self.channel)?,
            recipient: self.recipient,
            subject: self.subject,
            content: self.content,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct MessageQueryRequest {
    pub channel: Option<String>,
    pub recipient: Option<String>,
    pub status: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

impl MessageQueryRequest {
    pub fn into_query(self) -> Result<MessageQuery, EntityError> {
        Ok(MessageQuery {
            channel: match self.channel {
                Some(c) => Some(MessageChannel::from_str(&c)?),
                None => None,
            },
            recipient: self.recipient,
            status: self.status,
            page: self.page.unwrap_or(1),
            page_size: self.page_size.unwrap_or(20),
        })
    }
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub id: String,
    pub channel: String,
    pub recipient: String,
    pub subject: String,
    pub content: String,
    pub status: String,
    pub error: Option<String>,
    pub created_at: u64,
}
