use async_trait::async_trait;

use crate::models::message::Message;
use crate::repositories::RepositoryError;

#[derive(Debug, Clone)]
pub struct MessagePage {
    pub messages: Vec<Message>,
    pub next_cursor: Option<uuid::Uuid>,
}

#[async_trait]
pub trait MessageRepository: Send + Sync + 'static {
    async fn create_message(&self, message: &Message) -> Result<Message, RepositoryError>;
    async fn count_by_user_id(&self, user_id: &uuid::Uuid) -> Result<u64, RepositoryError>;
    async fn find_by_provider_message_id(
        &self,
        provider_message_id: &str,
    ) -> Result<Message, RepositoryError>;
    async fn list_by_conversation_id(
        &self,
        user_id: &uuid::Uuid,
        conversation_id: &uuid::Uuid,
    ) -> Result<Vec<Message>, RepositoryError>;
    async fn list_page_by_conversation_id(
        &self,
        user_id: &uuid::Uuid,
        conversation_id: &uuid::Uuid,
        cursor: Option<&uuid::Uuid>,
        limit: usize,
    ) -> Result<MessagePage, RepositoryError>;
    async fn update_message(&self, message: &Message) -> Result<Message, RepositoryError>;
}
