use time::OffsetDateTime;

#[derive(Debug, bon::Builder)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub hash: String,
    pub salt: String,
    pub email_verified: bool,
    pub email_verified_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
