//! Authentication-related DTOs for HTTP requests and responses.

use serde::{Deserialize, Serialize};

/// Login request DTO
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

impl From<LoginRequest> for application::commands::LoginCommand {
    fn from(req: LoginRequest) -> Self {
        Self {
            email: req.email,
            password: req.password,
        }
    }
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

/// Signup success response
#[derive(Debug, Serialize)]
pub struct SignupSuccessResponse {
    pub id: String,
    pub email: String,
}
