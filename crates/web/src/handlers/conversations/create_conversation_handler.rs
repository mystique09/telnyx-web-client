use std::sync::Arc;

use actix_session::Session;
use actix_web::{HttpRequest, HttpResponse, Responder, http::header::LOCATION, web};
use domain::{
    models::conversation::Conversation, repositories::RepositoryError,
    repositories::conversation_repository::ConversationRepository,
};
use time::OffsetDateTime;
use tracing::error;

use crate::{
    dto::{CreateConversationRequest, CreateConversationResponse, FlashProps},
    flash::set_flash,
    session::get_user_id,
};

pub async fn handle_create_conversation(
    req: HttpRequest,
    create_req: web::Json<CreateConversationRequest>,
    session: Session,
    conversation_repository: web::Data<Arc<dyn ConversationRepository>>,
) -> impl Responder {
    let Some(user_id) = session_user_id(&session) else {
        return HttpResponse::Found()
            .append_header((LOCATION, "/auth/login"))
            .finish();
    };

    let now = OffsetDateTime::now_utc();
    let conversation_id = uuid::Uuid::now_v7();
    let conversation = Conversation::builder()
        .id(conversation_id)
        .phone_number_id(create_req.phone_number_id)
        .user_id(user_id)
        .last_message_at(now)
        .created_at(now)
        .updated_at(now)
        .build();

    match conversation_repository
        .create_conversation(&conversation)
        .await
    {
        Ok(_) => {
            if req.headers().contains_key("x-inertia") {
                set_flash(&session, FlashProps::success("Conversation created."));
                return HttpResponse::Found()
                    .append_header((LOCATION, format!("/conversations/{}", conversation_id)))
                    .finish();
            }

            HttpResponse::Created().json(CreateConversationResponse {
                id: conversation_id,
            })
        }
        Err(err) => {
            error!(
                "failed to create conversation for user {} and phone_number {}: {}",
                user_id, create_req.phone_number_id, err
            );

            match err {
                RepositoryError::ConstraintViolation(_) | RepositoryError::DatabaseError(_) => {
                    HttpResponse::BadRequest().finish()
                }
                RepositoryError::NotFound | RepositoryError::UnexpectedError(_) => {
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    }
}

fn session_user_id(session: &Session) -> Option<uuid::Uuid> {
    get_user_id(session).and_then(|id| uuid::Uuid::parse_str(&id).ok())
}
