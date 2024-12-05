use super::repository::IInternalMessageRepository;
use super::dto::InternalMessageResponse;
use crate::errors::Result;

pub struct InternalMessageService<T: IInternalMessageRepository> {
    repo: T,
}

impl<T: IInternalMessageRepository> InternalMessageService<T> {
    pub fn new(repo: T) -> Self {
        Self { repo }
    }

    pub async fn get_my_messages(
        &self,
        recipient: String,
        page: Option<i64>,
        page_size: Option<i64>,
        status: Option<String>,
    ) -> Result<Vec<InternalMessageResponse>> {
        let page = page.unwrap_or(1);
        let page_size = page_size.unwrap_or(20);
        let skip = ((page - 1) * page_size) as u64;

        let messages = self
            .repo
            .find_by_recipient_with_filter(&recipient, status, skip, page_size)
            .await?;

        Ok(messages
            .into_iter()
            .map(|m| InternalMessageResponse {
                id: m.base.id,
                subject: m.subject,
                content: m.content,
                status: m.status,
                created_at: m.base.created_at,
            })
            .collect())
    }

    pub async fn mark_as_read(&self, id: String, recipient: String) -> Result<()> {
        let updated = self.repo.mark_as_read(&id, &recipient).await?;
        if !updated {
            return Err("消息不存在或无权限".into());
        }
        Ok(())
    }
}
