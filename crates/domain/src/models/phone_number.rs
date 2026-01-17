use time::OffsetDateTime;

#[derive(Debug, bon::Builder)]
pub struct PhoneNumber {
    pub id: uuid::Uuid,
    pub name: String,
    pub phone: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
