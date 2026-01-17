use time::OffsetDateTime;

#[derive(Debug, bon::Builder)]
pub struct ResetPassword {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub token: String,
    pub consumed: bool,
    pub consumed_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
