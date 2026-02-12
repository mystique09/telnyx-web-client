pub mod create_phone_number_handler;
pub mod delete_phone_number_handler;
pub mod get_phone_number_handler;
pub mod list_phone_numbers_handler;

use actix_web::{dev::HttpServiceFactory, web};

use crate::handlers::phone_numbers::{
    create_phone_number_handler::handle_create_phone_number,
    delete_phone_number_handler::handle_delete_phone_number,
    get_phone_number_handler::handle_get_phone_number,
    list_phone_numbers_handler::handle_list_phone_numbers,
};
use crate::middlewares::auth::ProtectedMiddleware;

pub fn build_phone_numbers_service() -> impl HttpServiceFactory {
    web::scope("/phone-numbers")
        .wrap(ProtectedMiddleware::new())
        .route("", web::get().to(handle_list_phone_numbers))
        .route("", web::post().to(handle_create_phone_number))
        .route("/{id}", web::get().to(handle_get_phone_number))
        .route("/{id}", web::delete().to(handle_delete_phone_number))
}
