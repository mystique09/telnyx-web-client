//! Authentication-related DTOs for HTTP requests and responses.

use serde::{Deserialize, Serialize};

/// Login request DTO
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Login response DTO
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub id: String,
    pub email: String,
}

impl LoginResponse {
    pub fn new(id: String, email: String) -> Self {
        Self { id, email }
    }
}

/// Forgot password request DTO
#[derive(Debug, Deserialize)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

/// Reset password request DTO
#[derive(Debug, Deserialize)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub password: String,
    pub password_confirmation: String,
}
