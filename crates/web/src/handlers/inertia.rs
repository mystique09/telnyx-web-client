use actix_inertia::inertia_responder::InertiaResponder;
use actix_web::{HttpRequest, Responder};

use crate::{Empty, inertia::response_with_html};

pub(crate) async fn version(req: HttpRequest) -> impl Responder {
    if req.headers().contains_key("x-inertia") {
        InertiaResponder::new("VersionPage", Empty).respond_to(&req)
    } else {
        response_with_html(&req, Empty, "VersionPage".to_string())
    }
}
