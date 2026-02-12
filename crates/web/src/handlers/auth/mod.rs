pub mod forgot_password_handler;
pub mod login_handler;
pub mod logout_handler;
pub mod reset_password_handler;
pub mod signup_handler;

use actix_web::{dev::HttpServiceFactory, web};

use crate::{
    handlers::auth::{
        forgot_password_handler::render_forgot_password,
        login_handler::{handle_login, render_login},
        logout_handler::handle_logout,
        reset_password_handler::render_reset_password,
        signup_handler::{handle_signup, render_signup},
    },
    middlewares::auth::{GuestMiddleware, ProtectedMiddleware},
};

pub fn build_auth_service() -> impl HttpServiceFactory {
    web::scope("/auth")
        .service(
            web::resource("/login")
                .wrap(GuestMiddleware::new())
                .route(web::get().to(render_login))
                .route(web::post().to(handle_login)),
        )
        .service(
            web::resource("/signup")
                .wrap(GuestMiddleware::new())
                .route(web::get().to(render_signup))
                .route(web::post().to(handle_signup)),
        )
        .service(
            web::resource("/forgot-password")
                .wrap(GuestMiddleware::new())
                .route(web::get().to(render_forgot_password)),
        )
        .service(
            web::resource("/reset-password")
                .wrap(GuestMiddleware::new())
                .route(web::get().to(render_reset_password)),
        )
        .service(
            web::resource("/logout")
                .wrap(ProtectedMiddleware::new())
                .route(web::post().to(handle_logout)),
        )
}
