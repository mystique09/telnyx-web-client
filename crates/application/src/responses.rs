use time::{OffsetDateTime, UtcDateTime};
use uuid::Uuid;

#[derive(Debug)]
pub struct SignupResult {
    pub id: Uuid,
    pub email: String,
    pub created_at: OffsetDateTime,
}

#[derive(Debug)]
pub struct LoginResult {
    pub id: Uuid,
    pub email: String,
    pub email_verified: bool,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: UtcDateTime,
}

#[derive(Debug)]
pub struct CreateConversationResult {
    pub id: Uuid,
}

#[derive(Debug)]
pub struct CreateMessageResult {
    pub message: domain::models::message::Message,
}

#[derive(Debug)]
pub struct CreatePhoneNumberResult {
    pub id: Uuid,
}

#[derive(Debug)]
pub struct DashboardAnalyticsResult {
    pub total_conversations: u64,
    pub total_messages: u64,
    pub total_phone_numbers: u64,
}

#[derive(Debug)]
pub struct DashboardHomeResult {
    pub phone_numbers: Vec<domain::models::phone_number::PhoneNumber>,
    pub analytics: DashboardAnalyticsResult,
}
