use application::commands;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub id: Uuid,
    pub email: String,
}

impl LoginResponse {
    pub fn new(id: Uuid, email: String) -> Self {
        Self { id, email }
    }
}

#[derive(Debug, Deserialize)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub password: String,
    pub password_confirmation: String,
}
