use actix_inertia::inertia_responder::InertiaResponder;
use actix_web::{HttpRequest, Responder};

use crate::{Empty, inertia::response_with_html};

pub async fn login(req: HttpRequest) -> impl Responder {
    if req.headers().contains_key("x-inertia") {
        InertiaResponder::new("Login", Empty).respond_to(&req)
    } else {
        response_with_html(&req, Empty, "Login".to_string())
    }
}

pub async fn signup(req: HttpRequest) -> impl Responder {
    if req.headers().contains_key("x-inertia") {
        InertiaResponder::new("Signup", Empty).respond_to(&req)
    } else {
        response_with_html(&req, Empty, "Signup".to_string())
    }
}

pub async fn forgot_password(req: HttpRequest) -> impl Responder {
    if req.headers().contains_key("x-inertia") {
        InertiaResponder::new("ForgotPassword", Empty).respond_to(&req)
    } else {
        response_with_html(&req, Empty, "ForgotPassword".to_string())
    }
}

pub async fn reset_password(req: HttpRequest) -> impl Responder {
    if req.headers().contains_key("x-inertia") {
        InertiaResponder::new("ResetPassword", Empty).respond_to(&req)
    } else {
        response_with_html(&req, Empty, "ResetPassword".to_string())
    }
}
