use std::sync::Arc;

use actix_session::Session;
use actix_web::{HttpRequest, HttpResponse, Responder, http::header::LOCATION, web};
use application::commands::CreatePhoneNumberCommand;
use application::usecases::UsecaseError;
use application::usecases::create_phone_number_usecase::CreatePhoneNumberUsecase;
use domain::repositories::phone_number_repository::PhoneNumberRepository;
use tracing::error;

use crate::{
    dto::{CreatePhoneNumberRequest, CreatePhoneNumberResponse, FlashProps},
    flash::set_flash,
    session::get_user_id,
};

pub async fn handle_create_phone_number(
    req: HttpRequest,
    create_req: web::Json<CreatePhoneNumberRequest>,
    session: Session,
    phone_number_repository: web::Data<Arc<dyn PhoneNumberRepository>>,
) -> impl Responder {
    let Some(user_id) = session_user_id(&session) else {
        return HttpResponse::Unauthorized().finish();
    };

    let create_phone_number_usecase = CreatePhoneNumberUsecase::builder()
        .phone_number_repository(phone_number_repository.get_ref().clone())
        .build();
    let cmd = CreatePhoneNumberCommand {
        user_id,
        name: create_req.name.clone(),
        phone: create_req.phone.clone(),
    };

    match create_phone_number_usecase.execute(cmd).await {
        Ok(result) => {
            if req.headers().contains_key("x-inertia") {
                set_flash(&session, FlashProps::success("Phone number added."));
                return HttpResponse::Found()
                    .append_header((LOCATION, "/"))
                    .finish();
            }

            HttpResponse::Created().json(CreatePhoneNumberResponse { id: result.id })
        }
        Err(err) => {
            error!(
                "failed to create phone number for user {} and phone {}: {}",
                user_id, create_req.phone, err
            );

            match err {
                UsecaseError::Database(_) => {
                    if req.headers().contains_key("x-inertia") {
                        set_flash(
                            &session,
                            FlashProps::error(
                                "Unable to add phone number. Check for duplicates and try again.",
                            ),
                        );
                        return HttpResponse::Found()
                            .append_header((LOCATION, "/"))
                            .finish();
                    }

                    HttpResponse::BadRequest().finish()
                }
                _ => {
                    if req.headers().contains_key("x-inertia") {
                        set_flash(
                            &session,
                            FlashProps::error("Unable to add phone number right now."),
                        );
                        return HttpResponse::Found()
                            .append_header((LOCATION, "/"))
                            .finish();
                    }

                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    }
}

fn session_user_id(session: &Session) -> Option<uuid::Uuid> {
    get_user_id(session).and_then(|id| uuid::Uuid::parse_str(&id).ok())
}
