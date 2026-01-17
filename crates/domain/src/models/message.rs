use time::OffsetDateTime;

#[derive(Debug, bon::Builder)]
pub struct Message {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub content: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
