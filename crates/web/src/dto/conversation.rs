use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateConversationRequest {
    #[serde(alias = "phoneNumberId")]
    pub phone_number_id: uuid::Uuid,
}

#[derive(Debug, Serialize)]
pub struct CreateConversationResponse {
    pub id: uuid::Uuid,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationProps {
    pub id: uuid::Uuid,
    pub phone_number_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub last_message_at: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<&domain::models::conversation::Conversation> for ConversationProps {
    fn from(value: &domain::models::conversation::Conversation) -> Self {
        Self {
            id: value.id,
            phone_number_id: value.phone_number_id,
            user_id: value.user_id,
            last_message_at: value.last_message_at.to_string(),
            created_at: value.created_at.to_string(),
            updated_at: value.updated_at.to_string(),
        }
    }
}
