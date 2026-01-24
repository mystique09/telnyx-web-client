//! Data Transfer Objects for HTTP layer.
//!
//! Contains request/response structs for API endpoints organized by domain.

pub mod auth;

pub use auth::{
    ForgotPasswordRequest, LoginRequest, LoginResponse, ResetPasswordRequest, SignupRequest,
};
