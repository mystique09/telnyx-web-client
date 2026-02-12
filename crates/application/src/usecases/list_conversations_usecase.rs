use std::sync::Arc;

use crate::usecases::UsecaseError;
use domain::{
    models::conversation::Conversation,
    repositories::conversation_repository::ConversationRepository,
};

#[derive(bon::Builder)]
pub struct ListConversationsUsecase {
    conversation_repository: Arc<dyn ConversationRepository>,
}

impl ListConversationsUsecase {
    pub async fn execute(&self, user_id: uuid::Uuid) -> Result<Vec<Conversation>, UsecaseError> {
        let conversations = self
            .conversation_repository
            .list_by_user_id(&user_id)
            .await?;

        Ok(conversations)
    }
}
