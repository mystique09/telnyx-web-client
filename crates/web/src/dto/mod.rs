//! Data Transfer Objects for HTTP layer.
//!
//! Contains request/response structs for API endpoints organized by domain.

pub mod auth;
pub mod conversation;
pub mod flash;

pub use auth::{
    ForgotPasswordRequest, LoginErrorProps, LoginRequest, LoginResponse, ResetPasswordRequest,
    SignupErrorProps, SignupRequest,
};
pub use conversation::{ConversationProps, CreateConversationRequest, CreateConversationResponse};
pub use flash::FlashProps;
