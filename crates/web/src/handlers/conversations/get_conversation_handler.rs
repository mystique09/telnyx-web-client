use actix_inertia::inertia_responder::InertiaResponder;
use actix_session::Session;
use actix_web::{HttpRequest, Responder, web};
use serde::Serialize;

use crate::{
    dto::FlashProps,
    flash::{clear_flash, extract_flash},
    inertia::response_with_html,
};

#[derive(Debug, Serialize, bon::Builder)]
struct ConversationPageProps {
    pub flash: Option<FlashProps>,
}

pub async fn render_get_conversation(
    req: HttpRequest,
    path: web::Path<String>,
    session: Session,
) -> impl Responder {
    let _id = path.into_inner();
    let flash = extract_flash(&session);

    if flash.is_some() {
        clear_flash(&session);
    }

    let props = ConversationPageProps::builder().maybe_flash(flash).build();

    if req.headers().contains_key("x-inertia") {
        InertiaResponder::new("Conversations", props).respond_to(&req)
    } else {
        response_with_html(&req, props, "Conversations".to_string())
    }
}
