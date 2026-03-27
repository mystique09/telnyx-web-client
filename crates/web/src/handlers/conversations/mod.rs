pub mod create_conversation_handler;
pub mod create_message_handler;
pub mod delete_conversation_handler;
pub mod get_conversation_handler;
pub mod list_conversation_messages_handler;
pub mod list_conversations_handler;

use actix_web::{dev::HttpServiceFactory, web};

use crate::handlers::conversations::{
    create_conversation_handler::handle_create_conversation,
    create_message_handler::handle_create_message,
    delete_conversation_handler::handle_delete_conversation,
    get_conversation_handler::render_get_conversation,
    list_conversation_messages_handler::handle_list_conversation_messages,
    list_conversations_handler::render_list_conversations,
};
use crate::middlewares::auth::ProtectedMiddleware;

pub const MESSAGE_PAGE_SIZE: usize = 10;
pub const MAX_MESSAGE_PAGE_SIZE: usize = 50;

pub fn build_conversations_service() -> impl HttpServiceFactory {
    web::scope("/conversations")
        .wrap(ProtectedMiddleware::new())
        .route("", web::get().to(render_list_conversations))
        .route("", web::post().to(handle_create_conversation))
        .route("/{id}", web::get().to(render_get_conversation))
        .route("/{id}/messages", web::get().to(handle_list_conversation_messages))
        .route("/{id}/messages", web::post().to(handle_create_message))
        .route("/{id}", web::delete().to(handle_delete_conversation))
}
