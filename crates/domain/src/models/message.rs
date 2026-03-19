use time::OffsetDateTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    Inbound,
    Outbound,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageStatus {
    Pending,
    Queued,
    Delivered,
    Failed,
    Sent,
}

#[derive(Debug, Clone, bon::Builder)]
pub struct Message {
    pub id: uuid::Uuid,
    pub conversation_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub message_type: MessageType,
    pub status: MessageStatus,
    pub provider_message_id: Option<String>,
    pub provider_status: Option<String>,
    pub provider_status_updated_at: Option<OffsetDateTime>,
    pub provider_error_code: Option<String>,
    pub provider_error_detail: Option<String>,
    pub from_number: String,
    pub content: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
