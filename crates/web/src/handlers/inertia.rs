use actix_web::{HttpRequest, Responder};

use crate::{Empty, inertia::Page};

pub(crate) async fn version(req: HttpRequest) -> impl Responder {
    Page::builder()
        .req(req)
        .name("VersionPage")
        .props(Empty)
        .build()
        .to_responder()
}
