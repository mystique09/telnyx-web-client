use async_trait::async_trait;

use crate::{models::conversation::Conversation, repositories::RepositoryError};

#[async_trait]
pub trait ConversationRepository: Send + Sync + 'static {
    async fn create_conversation(&self, conversation: &Conversation)
    -> Result<(), RepositoryError>;
    async fn update_conversation(&self, conversation: &Conversation)
    -> Result<(), RepositoryError>;
    async fn find_by_id(
        &self,
        user_id: &uuid::Uuid,
        id: &uuid::Uuid,
    ) -> Result<Conversation, RepositoryError>;
    async fn find_by_phone_number_and_recipient(
        &self,
        user_id: &uuid::Uuid,
        phone_number_id: &uuid::Uuid,
        recipient_phone_number: &str,
    ) -> Result<Conversation, RepositoryError>;
    async fn list_by_user_id(
        &self,
        user_id: &uuid::Uuid,
    ) -> Result<Vec<Conversation>, RepositoryError>;
    async fn delete_conversation(
        &self,
        user_id: &uuid::Uuid,
        id: &uuid::Uuid,
    ) -> Result<(), RepositoryError>;
}
