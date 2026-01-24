use time::{OffsetDateTime, UtcDateTime};
use uuid::Uuid;

pub struct SignupResult {
    pub id: Uuid,
    pub email: String,
    pub created_at: OffsetDateTime,
}

pub struct LoginResult {
    pub id: Uuid,
    pub email: String,
    pub email_verified: bool,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: UtcDateTime,
}
