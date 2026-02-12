use std::sync::Arc;

use time::OffsetDateTime;

use crate::{
    commands::CreateConversationCommand, responses::CreateConversationResult,
    usecases::UsecaseError,
};
use domain::{
    models::conversation::Conversation,
    repositories::conversation_repository::ConversationRepository,
};

#[derive(bon::Builder)]
pub struct CreateConversationUsecase {
    conversation_repository: Arc<dyn ConversationRepository>,
}

impl CreateConversationUsecase {
    pub async fn execute(
        &self,
        cmd: CreateConversationCommand,
    ) -> Result<CreateConversationResult, UsecaseError> {
        let now = OffsetDateTime::now_utc();
        let conversation_id = uuid::Uuid::now_v7();
        let conversation = Conversation::builder()
            .id(conversation_id)
            .phone_number_id(cmd.phone_number_id)
            .user_id(cmd.user_id)
            .last_message_at(now)
            .created_at(now)
            .updated_at(now)
            .build();

        self.conversation_repository
            .create_conversation(&conversation)
            .await?;

        Ok(CreateConversationResult {
            id: conversation_id,
        })
    }
}
