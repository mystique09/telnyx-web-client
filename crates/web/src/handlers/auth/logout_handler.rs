use crate::dto::auth::FlashProps;
use crate::flash::set_flash;
use crate::session::clear_authenticated;
use actix_session::Session;
use actix_web::{HttpResponse, Responder};

pub async fn handle_logout(session: Session) -> impl Responder {
    clear_authenticated(&session);

    set_flash(
        &session,
        FlashProps::success("You have been logged out successfully."),
    );

    HttpResponse::Found()
        .append_header((actix_web::http::header::LOCATION, "/auth/login"))
        .finish()
}
