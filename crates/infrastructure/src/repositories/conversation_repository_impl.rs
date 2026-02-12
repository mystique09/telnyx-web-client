use std::sync::Arc;

use domain::repositories::conversation_repository::ConversationRepository;
use domain::{models::conversation::Conversation, repositories::RepositoryError};

use rbatis::{RBatis, async_trait};
use rbs::value;

use crate::database;
use crate::repositories::RbsErrorExt;

#[derive(Debug, bon::Builder)]
pub struct ConversationRepositoryImpl {
    pool: Arc<RBatis>,
}

#[async_trait]
impl ConversationRepository for ConversationRepositoryImpl {
    async fn create_conversation(
        &self,
        conversation: &Conversation,
    ) -> Result<(), RepositoryError> {
        let new_conversation_db = database::models::conversation::Conversation::from(conversation);

        database::models::conversation::Conversation::insert(
            self.pool.as_ref(),
            &new_conversation_db,
        )
        .await
        .map_err(|e| e.to_repository_error())?;

        Ok(())
    }

    async fn find_by_id(
        &self,
        user_id: &uuid::Uuid,
        id: &uuid::Uuid,
    ) -> Result<Conversation, RepositoryError> {
        let conversation = database::models::conversation::Conversation::select_by_map(
            self.pool.as_ref(),
            value! { "id": id, "user_id": user_id },
        )
        .await
        .map_err(|e| e.to_repository_error())?
        .into_iter()
        .next()
        .ok_or(RepositoryError::NotFound)?;

        Ok(Conversation::from(&conversation))
    }

    async fn list_by_user_id(
        &self,
        user_id: &uuid::Uuid,
    ) -> Result<Vec<Conversation>, RepositoryError> {
        let records = database::models::conversation::Conversation::select_by_map(
            self.pool.as_ref(),
            value! { "user_id": user_id },
        )
        .await
        .map_err(|e| e.to_repository_error())?;

        let mut conversations = records.iter().map(Conversation::from).collect::<Vec<_>>();
        conversations.sort_by(|a, b| {
            b.last_message_at
                .cmp(&a.last_message_at)
                .then_with(|| b.id.cmp(&a.id))
        });

        Ok(conversations)
    }

    async fn delete_conversation(
        &self,
        user_id: &uuid::Uuid,
        id: &uuid::Uuid,
    ) -> Result<(), RepositoryError> {
        self.find_by_id(user_id, id).await?;

        database::models::conversation::Conversation::delete_by_map(
            self.pool.as_ref(),
            value! { "id": id, "user_id": user_id },
        )
        .await
        .map_err(|e| e.to_repository_error())?;

        Ok(())
    }
}
