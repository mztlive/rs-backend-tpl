use async_trait::async_trait;

use super::MessageSender;
use crate::errors::Result;

pub struct WebSocketSender;

impl WebSocketSender {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl MessageSender for WebSocketSender {
    async fn send(&self, recipient: &str, subject: &str, content: &str) -> Result<()> {
        // TODO: 实现实际的WebSocket消息发送逻辑
        Ok(())
    }
}
