use std::sync::Arc;

use actix_session::Session;
use actix_web::{HttpRequest, Responder, web};
use domain::repositories::RepositoryError;
use domain::repositories::conversation_repository::ConversationRepository;
use serde::Serialize;
use tracing::error;

use crate::{
    dto::{ConversationProps, FlashProps},
    flash::extract_flash,
    inertia::Page,
    session::get_user_id,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ConversationPageProps {
    pub flash: Option<FlashProps>,
    pub conversations: Vec<ConversationProps>,
    pub conversation: Option<ConversationProps>,
}

pub async fn render_get_conversation(
    req: HttpRequest,
    path: web::Path<uuid::Uuid>,
    session: Session,
    conversation_repository: web::Data<Arc<dyn ConversationRepository>>,
) -> impl Responder {
    let conversation_id = path.into_inner();
    let flash = extract_flash(&session);

    let (conversation, conversations) = match session_user_id(&session) {
        Some(user_id) => {
            let conversation = match conversation_repository
                .find_by_id(&user_id, &conversation_id)
                .await
            {
                Ok(item) => Some(ConversationProps::from(&item)),
                Err(RepositoryError::NotFound) => None,
                Err(err) => {
                    error!(
                        "failed to get conversation {} for user {}: {}",
                        conversation_id, user_id, err
                    );
                    None
                }
            };

            let conversations = match conversation_repository.list_by_user_id(&user_id).await {
                Ok(items) => items.iter().map(ConversationProps::from).collect(),
                Err(err) => {
                    error!("failed to list conversations for user {}: {}", user_id, err);
                    Vec::new()
                }
            };

            (conversation, conversations)
        }
        None => (None, Vec::new()),
    };

    Page::builder()
        .req(req)
        .name("Conversations")
        .props(ConversationPageProps {
            flash,
            conversations,
            conversation,
        })
        .build()
        .to_responder()
}

fn session_user_id(session: &Session) -> Option<uuid::Uuid> {
    get_user_id(session).and_then(|id| uuid::Uuid::parse_str(&id).ok())
}
