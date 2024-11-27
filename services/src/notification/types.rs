use serde::{Deserialize, Serialize};
use entities::MessageChannel;

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessageParams {
    pub channel: MessageChannel,
    pub recipient: String,
    pub subject: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageQuery {
    pub channel: Option<MessageChannel>,
    pub recipient: Option<String>,
    pub status: Option<String>,
    pub page: i64,
    pub page_size: i64,
} 