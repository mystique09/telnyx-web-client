use time::OffsetDateTime;

#[derive(Debug, Clone, bon::Builder)]
pub struct Conversation {
    pub id: uuid::Uuid,
    pub phone_number_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub recipient_phone_number: Option<String>,
    pub last_message_at: OffsetDateTime,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
