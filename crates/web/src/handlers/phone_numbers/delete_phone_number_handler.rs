use std::sync::Arc;

use actix_session::Session;
use actix_web::{HttpRequest, HttpResponse, Responder, http::header::LOCATION, web};
use application::usecases::UsecaseError;
use application::usecases::delete_phone_number_usecase::DeletePhoneNumberUsecase;
use domain::repositories::phone_number_repository::PhoneNumberRepository;
use tracing::error;

use crate::{dto::FlashProps, flash::set_flash, session::session_user_id};

pub async fn handle_delete_phone_number(
    req: HttpRequest,
    path: web::Path<uuid::Uuid>,
    session: Session,
    phone_number_repository: web::Data<Arc<dyn PhoneNumberRepository>>,
) -> impl Responder {
    let phone_number_id = path.into_inner();

    let Some(user_id) = session_user_id(&session) else {
        return HttpResponse::SeeOther()
            .append_header((LOCATION, "/auth/login"))
            .finish();
    };

    let delete_phone_number_usecase = DeletePhoneNumberUsecase::builder()
        .phone_number_repository(phone_number_repository.get_ref().clone())
        .build();

    match delete_phone_number_usecase
        .execute(user_id, phone_number_id)
        .await
    {
        Ok(_) => {
            if req.headers().contains_key("x-inertia") {
                set_flash(&session, FlashProps::success("Phone number deleted."));
                return HttpResponse::SeeOther()
                    .append_header((LOCATION, "/"))
                    .finish();
            }

            HttpResponse::NoContent().finish()
        }
        Err(UsecaseError::EntityNotFound) => {
            if req.headers().contains_key("x-inertia") {
                set_flash(&session, FlashProps::error("Phone number not found."));
                return HttpResponse::SeeOther()
                    .append_header((LOCATION, "/"))
                    .finish();
            }

            HttpResponse::NotFound().finish()
        }
        Err(err) => {
            error!(
                "failed to delete phone number {} for user {}: {}",
                phone_number_id, user_id, err
            );
            if req.headers().contains_key("x-inertia") {
                set_flash(
                    &session,
                    FlashProps::error("Unable to delete phone number right now."),
                );
                return HttpResponse::SeeOther()
                    .append_header((LOCATION, "/"))
                    .finish();
            }

            HttpResponse::InternalServerError().finish()
        }
    }
}
