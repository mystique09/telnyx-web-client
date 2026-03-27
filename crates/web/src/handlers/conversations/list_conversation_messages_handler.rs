use std::sync::Arc;

use actix_session::Session;
use actix_web::{HttpResponse, Responder, web};
use application::usecases::UsecaseError;
use application::usecases::get_conversation_usecase::GetConversationUsecase;
use application::usecases::list_messages_by_conversation_usecase::ListMessagesByConversationUsecase;
use domain::repositories::conversation_repository::ConversationRepository;
use domain::repositories::message_repository::MessageRepository;
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::{
    dto::{MessageProps, MessagesPageResponse},
    handlers::conversations::{MAX_MESSAGE_PAGE_SIZE, MESSAGE_PAGE_SIZE},
    session::session_user_id,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessagesPageQuery {
    cursor: Option<uuid::Uuid>,
    limit: Option<usize>,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

pub async fn handle_list_conversation_messages(
    path: web::Path<uuid::Uuid>,
    query: web::Query<MessagesPageQuery>,
    session: Session,
    conversation_repository: web::Data<Arc<dyn ConversationRepository>>,
    message_repository: web::Data<Arc<dyn MessageRepository>>,
) -> impl Responder {
    let Some(user_id) = session_user_id(&session) else {
        return HttpResponse::Unauthorized().finish();
    };

    let conversation_id = path.into_inner();
    let query = query.into_inner();
    let limit = query
        .limit
        .unwrap_or(MESSAGE_PAGE_SIZE)
        .clamp(1, MAX_MESSAGE_PAGE_SIZE);

    let get_conversation_usecase = GetConversationUsecase::builder()
        .conversation_repository(conversation_repository.get_ref().clone())
        .build();
    let list_messages_by_conversation_usecase = ListMessagesByConversationUsecase::builder()
        .message_repository(message_repository.get_ref().clone())
        .build();

    if let Err(err) = get_conversation_usecase.execute(user_id, conversation_id).await {
        return match err {
            UsecaseError::EntityNotFound => HttpResponse::NotFound().json(ErrorResponse {
                error: "Conversation not found.".to_owned(),
            }),
            _ => {
                error!(
                    "failed to validate conversation {} for user {}: {}",
                    conversation_id, user_id, err
                );
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Unable to load message history right now.".to_owned(),
                })
            }
        };
    }

    match list_messages_by_conversation_usecase
        .execute(user_id, conversation_id, query.cursor, limit)
        .await
    {
        Ok(page) => HttpResponse::Ok().json(MessagesPageResponse {
            messages: page.messages.iter().map(MessageProps::from).collect(),
            next_cursor: page.next_cursor,
        }),
        Err(UsecaseError::EntityNotFound) if query.cursor.is_some() => {
            HttpResponse::UnprocessableEntity().json(ErrorResponse {
                error: "Invalid message cursor.".to_owned(),
            })
        }
        Err(err) => {
            error!(
                "failed to list paginated messages for user {} and conversation {}: {}",
                user_id, conversation_id, err
            );
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Unable to load message history right now.".to_owned(),
            })
        }
    }
}
