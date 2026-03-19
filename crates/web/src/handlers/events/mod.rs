pub mod message_events_handler;

use actix_web::{dev::HttpServiceFactory, web};

use crate::handlers::events::message_events_handler::stream_message_events;

pub fn build_events_service() -> impl HttpServiceFactory {
    web::scope("/events").route("/messages", web::get().to(stream_message_events))
}
