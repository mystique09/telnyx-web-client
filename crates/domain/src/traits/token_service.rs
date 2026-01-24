use std::time::Duration;

use serde::{Deserialize, Serialize};
use time::UtcDateTime;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PasetoClaimPurpose {
    AccessToken,
    RefreshToken,
    PasswordReset,
    EmailVerification,
    FileUpload,
    FileRequest,
    FileAccess,
}

#[derive(thiserror::Error, Debug)]
pub enum TokenServiceError {
    #[error("Failed to generate token")]
    TokenGenerationFailed,
    #[error("Invalid token")]
    TokenInvalid,
    #[error("Failed to serialize token")]
    JsonSerializationError,
    #[error("Failed to deserialize token")]
    JsonDeserializationError,
    #[error("Expired token")]
    TokenExpired,
}

impl TryFrom<&str> for PasetoClaimPurpose {
    type Error = TokenServiceError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "access_token" => Ok(Self::AccessToken),
            "refresh_token" => Ok(Self::RefreshToken),
            "password_reset" => Ok(Self::PasswordReset),
            "email_verification" => Ok(Self::EmailVerification),
            "file_upload" => Ok(Self::FileUpload),
            "file_request" => Ok(Self::FileRequest),
            "file_access" => Ok(Self::FileAccess),
            _ => Err(TokenServiceError::TokenInvalid),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasetoClaims {
    pub id: uuid::Uuid,
    pub email: String,
    pub role: String,
    pub exp: Duration,
    pub purpose: PasetoClaimPurpose,
}

impl PasetoClaims {
    pub fn new(
        id: uuid::Uuid,
        email: String,
        role: String,
        exp: Duration,
        purpose: PasetoClaimPurpose,
    ) -> Self {
        Self {
            id,
            email,
            role,
            exp,
            purpose,
        }
    }
}

pub trait TokenService: Send + Sync + 'static {
    fn generate_token(
        &self,
        claims: PasetoClaims,
        expiration: Duration,
    ) -> Result<(String, UtcDateTime), TokenServiceError>;
    fn validate_token(
        &self,
        token: String,
        purpose: PasetoClaimPurpose,
    ) -> Result<PasetoClaims, TokenServiceError>;
}
