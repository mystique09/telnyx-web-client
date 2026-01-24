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
    middlewares::auth::GuestMiddleware,
};

pub fn build_auth_service() -> impl HttpServiceFactory {
    web::scope("/auth")
        .wrap(GuestMiddleware::new())
        .route("/login", web::get().to(render_login))
        .route("/login", web::post().to(handle_login))
        .route("/signup", web::get().to(render_signup))
        .route("/signup", web::post().to(handle_signup))
        .route("/logout", web::post().to(handle_logout))
        .route("/forgot-password", web::get().to(render_forgot_password))
        .route("/reset-password", web::get().to(render_reset_password))
}
