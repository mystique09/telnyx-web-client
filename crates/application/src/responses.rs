use time::OffsetDateTime;
use uuid::Uuid;

pub struct SignupResult {
    pub id: Uuid,
    pub email: String,
    pub created_at: OffsetDateTime,
}
