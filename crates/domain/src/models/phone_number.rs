use time::OffsetDateTime;

#[derive(Debug, Clone, bon::Builder)]
pub struct PhoneNumber {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub name: String,
    pub phone: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
