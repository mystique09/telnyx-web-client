use std::sync::Arc;

use crate::usecases::UsecaseError;
use domain::{
    models::conversation::Conversation,
    repositories::conversation_repository::ConversationRepository,
};

#[derive(bon::Builder)]
pub struct GetConversationUsecase {
    conversation_repository: Arc<dyn ConversationRepository>,
}

impl GetConversationUsecase {
    pub async fn execute(
        &self,
        user_id: uuid::Uuid,
        conversation_id: uuid::Uuid,
    ) -> Result<Conversation, UsecaseError> {
        let conversation = self
            .conversation_repository
            .find_by_id(&user_id, &conversation_id)
            .await?;

        Ok(conversation)
    }
}
