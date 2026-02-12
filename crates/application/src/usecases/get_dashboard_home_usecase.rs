use std::sync::Arc;

use crate::{
    responses::{DashboardAnalyticsResult, DashboardHomeResult},
    usecases::UsecaseError,
};
use domain::repositories::{
    conversation_repository::ConversationRepository, message_repository::MessageRepository,
    phone_number_repository::PhoneNumberRepository,
};

#[derive(bon::Builder)]
pub struct GetDashboardHomeUsecase {
    conversation_repository: Arc<dyn ConversationRepository>,
    message_repository: Arc<dyn MessageRepository>,
    phone_number_repository: Arc<dyn PhoneNumberRepository>,
}

impl GetDashboardHomeUsecase {
    pub async fn execute(&self, user_id: uuid::Uuid) -> Result<DashboardHomeResult, UsecaseError> {
        let conversations = self
            .conversation_repository
            .list_by_user_id(&user_id)
            .await?;
        let total_messages = self.message_repository.count_by_user_id(&user_id).await?;
        let phone_numbers = self
            .phone_number_repository
            .list_by_user_id(&user_id)
            .await?;

        Ok(DashboardHomeResult {
            analytics: DashboardAnalyticsResult {
                total_conversations: conversations.len() as u64,
                total_messages,
                total_phone_numbers: phone_numbers.len() as u64,
            },
            phone_numbers,
        })
    }
}
