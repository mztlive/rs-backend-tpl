use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct GetMessagesRequest {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct InternalMessageResponse {
    pub id: String,
    pub subject: String,
    pub content: String,
    pub status: String,
    pub created_at: u64,
}

impl From<services::internal_message::InternalMessageResponse> for InternalMessageResponse {
    fn from(msg: services::internal_message::InternalMessageResponse) -> Self {
        Self {
            id: msg.id,
            subject: msg.subject,
            content: msg.content,
            status: format!("{:?}", msg.status),
            created_at: msg.created_at,
        }
    }
}
