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
        let recipient_phone_number = cmd.recipient_phone_number.trim();
        if recipient_phone_number.is_empty() {
            return Err(garde::Error::new("Recipient phone number is required").into());
        }

        let now = OffsetDateTime::now_utc();
        let conversation_id = uuid::Uuid::now_v7();
        let conversation = Conversation::builder()
            .id(conversation_id)
            .phone_number_id(cmd.phone_number_id)
            .user_id(cmd.user_id)
            .maybe_recipient_phone_number(Some(recipient_phone_number.to_owned()))
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
