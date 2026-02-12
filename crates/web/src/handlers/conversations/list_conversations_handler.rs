use std::sync::Arc;

use actix_session::Session;
use actix_web::{HttpRequest, Responder, web};
use domain::repositories::conversation_repository::ConversationRepository;
use domain::repositories::phone_number_repository::PhoneNumberRepository;
use serde::Serialize;
use tracing::error;

use crate::{
    dto::{ConversationProps, FlashProps, PhoneNumberProps},
    flash::extract_flash,
    inertia::Page,
    session::get_user_id,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ConversationsPageProps {
    pub flash: Option<FlashProps>,
    pub conversations: Vec<ConversationProps>,
    pub phone_numbers: Vec<PhoneNumberProps>,
}

pub async fn render_list_conversations(
    req: HttpRequest,
    session: Session,
    conversation_repository: web::Data<Arc<dyn ConversationRepository>>,
    phone_number_repository: web::Data<Arc<dyn PhoneNumberRepository>>,
) -> impl Responder {
    let flash = extract_flash(&session);

    let (conversations, phone_numbers) = match session_user_id(&session) {
        Some(user_id) => {
            let conversations = match conversation_repository.list_by_user_id(&user_id).await {
                Ok(items) => items.iter().map(ConversationProps::from).collect(),
                Err(err) => {
                    error!("failed to list conversations for user {}: {}", user_id, err);
                    Vec::new()
                }
            };

            let phone_numbers = match phone_number_repository.list_by_user_id(&user_id).await {
                Ok(items) => items.iter().map(PhoneNumberProps::from).collect(),
                Err(err) => {
                    error!("failed to list phone numbers for user {}: {}", user_id, err);
                    Vec::new()
                }
            };

            (conversations, phone_numbers)
        }
        None => (Vec::new(), Vec::new()),
    };

    Page::builder()
        .req(req)
        .name("Conversations")
        .props(ConversationsPageProps {
            flash,
            conversations,
            phone_numbers,
        })
        .build()
        .to_responder()
}

fn session_user_id(session: &Session) -> Option<uuid::Uuid> {
    get_user_id(session).and_then(|id| uuid::Uuid::parse_str(&id).ok())
}
