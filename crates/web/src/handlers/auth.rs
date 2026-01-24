use std::sync::Arc;

use actix_inertia::inertia_responder::InertiaResponder;
use actix_web::{HttpRequest, Responder, web};
use serde::Serialize;

use crate::{Empty, dto::SignupRequest, inertia::response_with_html, types::WebError};
use application::usecases::create_user_usecase::CreateUserUsecase;

/// Render login page - GET /login
pub async fn render_login(req: HttpRequest) -> impl Responder {
    if req.headers().contains_key("x-inertia") {
        InertiaResponder::new("Login", Empty).respond_to(&req)
    } else {
        response_with_html(&req, Empty, "Login".to_string())
    }
}

/// Render signup page - GET /signup
pub async fn render_signup(req: HttpRequest) -> impl Responder {
    if req.headers().contains_key("x-inertia") {
        InertiaResponder::new("Signup", Empty).respond_to(&req)
    } else {
        response_with_html(&req, Empty, "Signup".to_string())
    }
}

/// Process signup form - POST /signup
pub async fn handle_signup(
    signup_req: web::Json<SignupRequest>,
    create_user: web::Data<Arc<CreateUserUsecase>>,
) -> Result<web::Json<SignupSuccessResponse>, WebError> {
    let cmd = signup_req.into_inner().into();
    let result = create_user.execute(cmd).await?;

    Ok(web::Json(SignupSuccessResponse {
        id: result.id.to_string(),
        email: result.email,
    }))
}

/// Render forgot password page - GET /forgot-password
pub async fn render_forgot_password(req: HttpRequest) -> impl Responder {
    if req.headers().contains_key("x-inertia") {
        InertiaResponder::new("ForgotPassword", Empty).respond_to(&req)
    } else {
        response_with_html(&req, Empty, "ForgotPassword".to_string())
    }
}

/// Render reset password page - GET /reset-password
pub async fn render_reset_password(req: HttpRequest) -> impl Responder {
    if req.headers().contains_key("x-inertia") {
        InertiaResponder::new("ResetPassword", Empty).respond_to(&req)
    } else {
        response_with_html(&req, Empty, "ResetPassword".to_string())
    }
}

/// Signup success response
#[derive(Debug, Serialize)]
pub struct SignupSuccessResponse {
    pub id: String,
    pub email: String,
}
