use actix_inertia::inertia_responder::InertiaResponder;
use actix_web::{HttpRequest, Responder};

use crate::{Empty, inertia::response_with_html};

/// Render reset password page - GET /reset-password
pub async fn render_reset_password(req: HttpRequest) -> impl Responder {
    if req.headers().contains_key("x-inertia") {
        InertiaResponder::new("ResetPassword", Empty).respond_to(&req)
    } else {
        response_with_html(&req, Empty, "ResetPassword".to_string())
    }
}
