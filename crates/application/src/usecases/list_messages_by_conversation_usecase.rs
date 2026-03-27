use std::sync::Arc;

use crate::usecases::UsecaseError;
use domain::repositories::message_repository::{MessagePage, MessageRepository};

#[derive(bon::Builder)]
pub struct ListMessagesByConversationUsecase {
    message_repository: Arc<dyn MessageRepository>,
}

impl ListMessagesByConversationUsecase {
    pub async fn execute(
        &self,
        user_id: uuid::Uuid,
        conversation_id: uuid::Uuid,
        cursor: Option<uuid::Uuid>,
        limit: usize,
    ) -> Result<MessagePage, UsecaseError> {
        self.message_repository
            .list_page_by_conversation_id(&user_id, &conversation_id, cursor.as_ref(), limit)
            .await
            .map_err(Into::into)
    }
}
