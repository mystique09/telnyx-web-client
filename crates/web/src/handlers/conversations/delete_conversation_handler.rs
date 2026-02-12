use std::sync::Arc;

use actix_session::Session;
use actix_web::{HttpRequest, HttpResponse, Responder, http::header::LOCATION, web};
use application::usecases::UsecaseError;
use application::usecases::delete_conversation_usecase::DeleteConversationUsecase;
use domain::repositories::conversation_repository::ConversationRepository;
use tracing::error;

use crate::{dto::FlashProps, flash::set_flash, session::get_user_id};

pub async fn handle_delete_conversation(
    req: HttpRequest,
    path: web::Path<uuid::Uuid>,
    session: Session,
    conversation_repository: web::Data<Arc<dyn ConversationRepository>>,
) -> impl Responder {
    let conversation_id = path.into_inner();

    let Some(user_id) = session_user_id(&session) else {
        return HttpResponse::Found()
            .append_header((LOCATION, "/auth/login"))
            .finish();
    };

    let delete_conversation_usecase = DeleteConversationUsecase::builder()
        .conversation_repository(conversation_repository.get_ref().clone())
        .build();

    match delete_conversation_usecase
        .execute(user_id, conversation_id)
        .await
    {
        Ok(_) => {
            if req.headers().contains_key("x-inertia") {
                set_flash(&session, FlashProps::success("Conversation deleted."));
                return HttpResponse::Found()
                    .append_header((LOCATION, "/conversations"))
                    .finish();
            }

            HttpResponse::NoContent().finish()
        }
        Err(UsecaseError::EntityNotFound) => {
            if req.headers().contains_key("x-inertia") {
                set_flash(&session, FlashProps::error("Conversation not found."));
                return HttpResponse::Found()
                    .append_header((LOCATION, "/conversations"))
                    .finish();
            }

            HttpResponse::NotFound().finish()
        }
        Err(err) => {
            error!(
                "failed to delete conversation {} for user {}: {}",
                conversation_id, user_id, err
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}

fn session_user_id(session: &Session) -> Option<uuid::Uuid> {
    get_user_id(session).and_then(|id| uuid::Uuid::parse_str(&id).ok())
}
