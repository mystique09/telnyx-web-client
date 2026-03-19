use garde::Validate;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Validate, Clone)]
pub struct SignupCommand {
    #[garde(email)]
    pub email: String,

    #[garde(length(min = 8))]
    pub password: String,

    #[garde(skip)]
    pub password_confirmation: String,
}

impl SignupCommand {
    pub fn validate_passwords_match(&self) -> Result<(), garde::Error> {
        if self.password != self.password_confirmation {
            Err(garde::Error::new("Passwords do not match"))
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Validate, Clone)]
pub struct LoginCommand {
    #[garde(email)]
    pub email: String,

    #[garde(length(min = 1))]
    pub password: String,
}

#[derive(Debug, Validate, Clone)]
pub struct ForgotPasswordCommand {
    #[garde(email)]
    pub email: String,
}

#[derive(Debug, Validate, Clone)]
pub struct ResetPasswordCommand {
    #[garde(length(min = 32))]
    pub token: String,

    #[garde(length(min = 8))]
    pub password: String,

    #[garde(skip)]
    pub password_confirmation: String,
}

impl ResetPasswordCommand {
    pub fn validate_passwords_match(&self) -> Result<(), garde::Error> {
        if self.password != self.password_confirmation {
            Err(garde::Error::new("Passwords do not match"))
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Clone)]
pub struct CreateConversationCommand {
    pub user_id: Uuid,
    pub phone_number_id: Uuid,
    pub recipient_phone_number: String,
}

#[derive(Debug, Clone)]
pub struct CreatePhoneNumberCommand {
    pub user_id: Uuid,
    pub name: String,
    pub phone: String,
}

#[derive(Debug, Clone)]
pub struct CreateMessageCommand {
    pub user_id: Uuid,
    pub conversation_id: Uuid,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct ProcessTelnyxWebhookCommand {
    pub event_id: String,
    pub event_type: String,
    pub occurred_at: OffsetDateTime,
    pub payload: TelnyxWebhookMessagePayload,
    pub raw_payload: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct TelnyxWebhookMessagePayload {
    pub provider_message_id: String,
    pub from_phone_number: Option<String>,
    pub to: Vec<TelnyxWebhookMessageParticipant>,
    pub text: Option<String>,
    pub received_at: Option<OffsetDateTime>,
    pub sent_at: Option<OffsetDateTime>,
    pub completed_at: Option<OffsetDateTime>,
    pub errors: Vec<TelnyxWebhookMessageError>,
}

#[derive(Debug, Clone)]
pub struct TelnyxWebhookMessageParticipant {
    pub phone_number: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TelnyxWebhookMessageError {
    pub code: Option<String>,
    pub detail: Option<String>,
    pub title: Option<String>,
}
