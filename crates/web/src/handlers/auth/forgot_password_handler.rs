use actix_web::{HttpRequest, Responder};

use crate::{Empty, inertia::Page};

/// Render forgot password page - GET /forgot-password
pub async fn render_forgot_password(req: HttpRequest) -> impl Responder {
    Page::builder()
        .req(req)
        .name("ForgotPassword")
        .props(Empty)
        .build()
        .to_responder()
}
