use actix_web::{HttpRequest, Responder};

use crate::{Empty, inertia::Page};

/// Render reset password page - GET /reset-password
pub async fn render_reset_password(req: HttpRequest) -> impl Responder {
    Page::builder()
        .req(req)
        .name("ResetPassword")
        .props(Empty)
        .build()
        .to_responder()
}
