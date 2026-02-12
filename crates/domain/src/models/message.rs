use time::OffsetDateTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    Inbound,
    Outbound,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageStatus {
    Pending,
    Delivered,
    Failed,
    Sent,
}

#[derive(Debug, bon::Builder)]
pub struct Message {
    pub id: uuid::Uuid,
    pub conversation_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub message_type: MessageType,
    pub status: MessageStatus,
    pub from_number: String,
    pub content: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
