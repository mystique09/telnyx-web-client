use actix_inertia::inertia_responder::InertiaResponder;
use actix_web::{HttpRequest, Responder};

use crate::{Empty, inertia::response_with_html};

/// Render forgot password page - GET /forgot-password
pub async fn render_forgot_password(req: HttpRequest) -> impl Responder {
    if req.headers().contains_key("x-inertia") {
        InertiaResponder::new("ForgotPassword", Empty).respond_to(&req)
    } else {
        response_with_html(&req, Empty, "ForgotPassword".to_string())
    }
}
