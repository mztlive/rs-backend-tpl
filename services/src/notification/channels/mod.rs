mod email;
mod internal;
mod sms;
mod websocket;

pub use email::EmailSender;
pub use internal::InternalMessageSender;
pub use sms::SMSSender;
pub use websocket::WebSocketSender;

use super::super::errors::Result;

#[async_trait::async_trait]
pub trait MessageSender: Send + Sync {
    async fn send(&self, recipient: &str, subject: &str, content: &str) -> Result<()>;
}
