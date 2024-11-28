use crate::{errors::Result, internal_message::IInternalMessageRepository};
use entities::{Message, MessageChannel, MessageStatus};

use super::{
    channels::{EmailSender, InternalMessageSender, MessageSender, SMSSender, WebSocketSender},
    types::{MessageQuery, SendMessageParams},
    IMessageRepository,
};

pub struct MessageService<T, TM>
where
    T: IMessageRepository,
    TM: IInternalMessageRepository,
{
    repo: T,
    email_sender: EmailSender,
    sms_sender: SMSSender,
    ws_sender: WebSocketSender,
    internal_sender: InternalMessageSender<TM>,
}

impl<T: IMessageRepository, TM: IInternalMessageRepository> MessageService<T, TM> {
    pub fn new(repo: T, internal_msg_repo: TM) -> Self {
        Self {
            repo,
            email_sender: EmailSender::new(),
            sms_sender: SMSSender::new(),
            ws_sender: WebSocketSender::new(),
            internal_sender: InternalMessageSender::new(internal_msg_repo),
        }
    }

    pub async fn new_message(&self, params: SendMessageParams) -> Result<()> {
        let id = libs::next_id().await;
        let message = Message::new(
            id,
            params.channel,
            params.recipient,
            params.subject,
            params.content,
        );

        // 保存消息记录
        self.repo.create(&message).await?;
        self.send_message(message).await?;
        Ok(())
    }

    pub async fn send_message(&self, mut message: Message) -> Result<()> {
        let result = match message.channel {
            MessageChannel::Email => {
                self.email_sender
                    .send(&message.recipient, &message.subject, &message.content)
                    .await
            }
            MessageChannel::SMS => {
                self.sms_sender
                    .send(&message.recipient, &message.subject, &message.content)
                    .await
            }
            MessageChannel::WebSocket => {
                self.ws_sender
                    .send(&message.recipient, &message.subject, &message.content)
                    .await
            }
            MessageChannel::InternalMessage => {
                self.internal_sender
                    .send(&message.recipient, &message.subject, &message.content)
                    .await
            }
        };

        // 更新发送状态
        message.status = match result {
            Ok(_) => MessageStatus::Sent,
            Err(e) => {
                message.error = Some(e.to_string());
                MessageStatus::Failed
            }
        };

        self.repo.update(&message).await?;
        Ok(())
    }

    pub async fn get_message_list(&self, query: MessageQuery) -> Result<Vec<Message>> {
        Ok(self.repo.query(query).await?)
    }

    pub async fn retry_by_id(&self, id: &str) -> Result<()> {
        let message = self.repo.find_by_id(id).await?.ok_or("消息不存在")?;
        self.retry_message(message).await
    }

    pub async fn retry_message(&self, message: Message) -> Result<()> {
        if message.status != MessageStatus::Failed {
            return Err("只能重试失败的消息".into());
        }

        self.send_message(message).await
    }

    pub async fn get_failed_messages(&self) -> Result<Vec<Message>> {
        Ok(self.repo.find_failed_messages().await?)
    }

    pub async fn get_pending_messages(&self) -> Result<Vec<Message>> {
        Ok(self.repo.find_pending_messages().await?)
    }
}
