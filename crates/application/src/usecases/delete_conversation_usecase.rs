use std::sync::Arc;

use crate::usecases::UsecaseError;
use domain::repositories::conversation_repository::ConversationRepository;

#[derive(bon::Builder)]
pub struct DeleteConversationUsecase {
    conversation_repository: Arc<dyn ConversationRepository>,
}

impl DeleteConversationUsecase {
    pub async fn execute(
        &self,
        user_id: uuid::Uuid,
        conversation_id: uuid::Uuid,
    ) -> Result<(), UsecaseError> {
        self.conversation_repository
            .delete_conversation(&user_id, &conversation_id)
            .await?;

        Ok(())
    }
}
