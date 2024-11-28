use serde::Serialize;
use entities::InternalMessageStatus;

#[derive(Debug, Serialize)]
pub struct InternalMessageResponse {
    pub id: String,
    pub subject: String,
    pub content: String,
    pub status: InternalMessageStatus,
    pub created_at: u64,
} 