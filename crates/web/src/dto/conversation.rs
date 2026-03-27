use serde::{Deserialize, Serialize};
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

#[derive(Debug, Deserialize)]
pub struct CreateConversationRequest {
    #[serde(alias = "phoneNumberId")]
    pub phone_number_id: uuid::Uuid,
    #[serde(alias = "recipientPhoneNumber")]
    pub recipient_phone_number: String,
}

#[derive(Debug, Serialize)]
pub struct CreateConversationResponse {
    pub id: uuid::Uuid,
}

#[derive(Debug, Deserialize)]
pub struct CreateMessageRequest {
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct CreateMessageResponse {
    pub message: MessageProps,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessagesPageResponse {
    pub messages: Vec<MessageProps>,
    pub next_cursor: Option<uuid::Uuid>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageEventProps {
    #[serde(rename = "type")]
    pub event_type: String,
    pub message: MessageProps,
    pub conversation: ConversationProps,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationProps {
    pub id: uuid::Uuid,
    pub phone_number_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub recipient_phone_number: Option<String>,
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
            recipient_phone_number: value.recipient_phone_number.to_owned(),
            last_message_at: format_datetime(value.last_message_at),
            created_at: format_datetime(value.created_at),
            updated_at: format_datetime(value.updated_at),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageProps {
    pub id: uuid::Uuid,
    pub conversation_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub message_type: &'static str,
    pub status: &'static str,
    pub provider_message_id: Option<String>,
    pub provider_status: Option<String>,
    pub provider_status_updated_at: Option<String>,
    pub provider_error_code: Option<String>,
    pub provider_error_detail: Option<String>,
    pub from_number: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<&domain::models::message::Message> for MessageProps {
    fn from(value: &domain::models::message::Message) -> Self {
        Self {
            id: value.id,
            conversation_id: value.conversation_id,
            user_id: value.user_id,
            message_type: match value.message_type {
                domain::models::message::MessageType::Inbound => "INBOUND",
                domain::models::message::MessageType::Outbound => "OUTBOUND",
            },
            status: match value.status {
                domain::models::message::MessageStatus::Pending => "pending",
                domain::models::message::MessageStatus::Queued => "queued",
                domain::models::message::MessageStatus::Delivered => "delivered",
                domain::models::message::MessageStatus::Failed => "failed",
                domain::models::message::MessageStatus::Sent => "sent",
            },
            provider_message_id: value.provider_message_id.to_owned(),
            provider_status: value.provider_status.to_owned(),
            provider_status_updated_at: value.provider_status_updated_at.map(format_datetime),
            provider_error_code: value.provider_error_code.to_owned(),
            provider_error_detail: value.provider_error_detail.to_owned(),
            from_number: value.from_number.to_owned(),
            content: value.content.to_owned(),
            created_at: format_datetime(value.created_at),
            updated_at: format_datetime(value.updated_at),
        }
    }
}

fn format_datetime(value: OffsetDateTime) -> String {
    value.format(&Rfc3339).unwrap_or_else(|_| value.to_string())
}
