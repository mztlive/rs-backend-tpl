use async_trait::async_trait;

use super::MessageSender;
use crate::errors::Result;

pub struct EmailSender;

impl EmailSender {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl MessageSender for EmailSender {
    async fn send(&self, recipient: &str, subject: &str, content: &str) -> Result<()> {
        Ok(())
    }
}
