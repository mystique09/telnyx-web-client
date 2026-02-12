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

pub struct CreateConversationResult {
    pub id: Uuid,
}

pub struct CreatePhoneNumberResult {
    pub id: Uuid,
}

pub struct DashboardAnalyticsResult {
    pub total_conversations: u64,
    pub total_messages: u64,
    pub total_phone_numbers: u64,
}

pub struct DashboardHomeResult {
    pub phone_numbers: Vec<domain::models::phone_number::PhoneNumber>,
    pub analytics: DashboardAnalyticsResult,
}
