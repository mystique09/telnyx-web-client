use std::sync::Arc;

use crate::usecases::UsecaseError;
use domain::{models::message::Message, repositories::message_repository::MessageRepository};

#[derive(bon::Builder)]
pub struct ListMessagesByConversationUsecase {
    message_repository: Arc<dyn MessageRepository>,
}

impl ListMessagesByConversationUsecase {
    pub async fn execute(
        &self,
        user_id: uuid::Uuid,
        conversation_id: uuid::Uuid,
    ) -> Result<Vec<Message>, UsecaseError> {
        self.message_repository
            .list_by_conversation_id(&user_id, &conversation_id)
            .await
            .map_err(Into::into)
    }
}
