pub mod get_conversation_handler;
pub mod list_conversations_handler;

use actix_web::{dev::HttpServiceFactory, web};

use crate::handlers::conversations::{
    get_conversation_handler::render_get_conversation,
    list_conversations_handler::render_list_conversations,
};
use crate::middlewares::auth::ProtectedMiddleware;

pub fn build_conversations_service() -> impl HttpServiceFactory {
    web::scope("/conversations")
        .wrap(ProtectedMiddleware::new())
        .route("", web::get().to(render_list_conversations))
        .route("/{id}", web::get().to(render_get_conversation))
}
