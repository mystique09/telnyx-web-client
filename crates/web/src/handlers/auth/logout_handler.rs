use crate::dto::auth::FlashProps;
use crate::flash::set_flash;
use actix_session::Session;
use actix_web::{HttpResponse, Responder};

pub async fn handle_logout(session: Session) -> impl Responder {
    session.clear();

    set_flash(
        &session,
        FlashProps::success("You have been logged out successfully."),
    );

    HttpResponse::SeeOther()
        .append_header((actix_web::http::header::LOCATION, "/auth/login"))
        .append_header((
            actix_web::http::header::SET_COOKIE,
            "access_token=; HttpOnly; Secure; Path=/; SameSite=Lax; Max-Age=0",
        ))
        .append_header((
            actix_web::http::header::SET_COOKIE,
            "refresh_token=; HttpOnly; Secure; Path=/; SameSite=Lax; Max-Age=0",
        ))
        .finish()
}
