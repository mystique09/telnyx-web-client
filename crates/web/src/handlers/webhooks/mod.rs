pub mod telnyx_messaging_webhook_handler;

use actix_web::{dev::HttpServiceFactory, web};

use crate::handlers::webhooks::telnyx_messaging_webhook_handler::handle_telnyx_messaging_webhook;

pub fn build_webhooks_service() -> impl HttpServiceFactory {
    web::scope("/webhooks").route(
        "/telnyx/messaging",
        web::post().to(handle_telnyx_messaging_webhook),
    )
}
