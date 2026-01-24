//! Data Transfer Objects for HTTP layer.
//!
//! Contains request/response structs for API endpoints organized by domain.

use application::commands;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod auth;

pub use auth::{ForgotPasswordRequest, LoginRequest, LoginResponse, ResetPasswordRequest};

/// Signup request DTO
#[derive(Debug, Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    pub password_confirmation: String,
}

impl From<SignupRequest> for commands::SignupCommand {
    fn from(req: SignupRequest) -> Self {
        Self {
            email: req.email,
            password: req.password,
            password_confirmation: req.password_confirmation,
        }
    }
}

/// Signup response DTO
#[derive(Debug, Serialize)]
pub struct SignupResponse {
    pub id: Uuid,
    pub email: String,
    pub created_at: String,
}

impl SignupResponse {
    pub fn new(id: Uuid, email: String, created_at: String) -> Self {
        Self {
            id,
            email,
            created_at,
        }
    }
}
