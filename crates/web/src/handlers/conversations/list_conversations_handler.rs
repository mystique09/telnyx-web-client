use actix_session::Session;
use actix_web::{HttpRequest, Responder};
use serde::Serialize;

use crate::{
    dto::FlashProps,
    flash::{clear_flash, extract_flash},
    inertia::Page,
};

#[derive(Debug, Serialize, bon::Builder)]
struct ConversationsPageProps {
    pub flash: Option<FlashProps>,
}

pub async fn render_list_conversations(req: HttpRequest, session: Session) -> impl Responder {
    let flash = extract_flash(&session);

    if flash.is_some() {
        clear_flash(&session);
    }

    let props = ConversationsPageProps::builder().maybe_flash(flash).build();

    Page::builder()
        .req(req)
        .name("Conversations")
        .props(props)
        .build()
        .to_responder()
}
