use async_trait::async_trait;

use super::MessageSender;

pub struct SMSSender;

use crate::errors::Result;

impl SMSSender {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl MessageSender for SMSSender {
    async fn send(&self, recipient: &str, subject: &str, content: &str) -> Result<()> {
        // TODO: 实现实际的短信发送逻辑
        Ok(())
    }
}
