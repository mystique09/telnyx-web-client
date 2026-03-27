use std::sync::Arc;

use domain::models::message::Message;
use domain::repositories::RepositoryError;
use domain::repositories::message_repository::{MessagePage, MessageRepository};

use rbatis::{RBatis, async_trait};
use rbs::value;

use crate::database;
use crate::database::models::{MessageSql, RdbcUuidExt, UuidExt};
use crate::repositories::RbsErrorExt;

#[derive(Debug, bon::Builder)]
pub struct MessageRepositoryImpl {
    pool: Arc<RBatis>,
}

#[async_trait]
impl MessageRepository for MessageRepositoryImpl {
    async fn create_message(&self, message: &Message) -> Result<Message, RepositoryError> {
        let new_message_db = database::models::message::Message::from(message);
        let created_message = Message::from(&new_message_db);

        MessageSql::insert_message(self.pool.as_ref(), &new_message_db)
            .await
            .map_err(|e| e.to_repository_error())?;

        Ok(created_message)
    }

    async fn count_by_user_id(&self, user_id: &uuid::Uuid) -> Result<u64, RepositoryError> {
        let user_id_db = user_id.into_db();
        let records = database::models::message::Message::select_by_map(
            self.pool.as_ref(),
            value! { "user_id": user_id_db },
        )
        .await
        .map_err(|e| e.to_repository_error())?;

        Ok(records.len() as u64)
    }

    async fn find_by_provider_message_id(
        &self,
        provider_message_id: &str,
    ) -> Result<Message, RepositoryError> {
        let record = database::models::message::Message::select_by_map(
            self.pool.as_ref(),
            value! { "provider_message_id": provider_message_id },
        )
        .await
        .map_err(|e| e.to_repository_error())?
        .into_iter()
        .next()
        .ok_or(RepositoryError::NotFound)?;

        Ok(Message::from(&record))
    }

    async fn list_by_conversation_id(
        &self,
        user_id: &uuid::Uuid,
        conversation_id: &uuid::Uuid,
    ) -> Result<Vec<Message>, RepositoryError> {
        let user_id_db = user_id.into_db();
        let conversation_id_db = conversation_id.into_db();
        let records = database::models::message::Message::select_by_map(
            self.pool.as_ref(),
            value! { "conversation_id": conversation_id_db, "user_id": user_id_db },
        )
        .await
        .map_err(|e| e.to_repository_error())?;

        let mut messages = records.iter().map(Message::from).collect::<Vec<_>>();
        messages.sort_by(|a, b| {
            a.created_at
                .cmp(&b.created_at)
                .then_with(|| a.id.cmp(&b.id))
        });

        Ok(messages)
    }

    async fn list_page_by_conversation_id(
        &self,
        user_id: &uuid::Uuid,
        conversation_id: &uuid::Uuid,
        cursor: Option<&uuid::Uuid>,
        limit: usize,
    ) -> Result<MessagePage, RepositoryError> {
        let page_size = limit.max(1);
        let query_limit = (page_size + 1) as i64;
        let user_id_db = user_id.into_db();
        let conversation_id_db = conversation_id.into_db();

        let (cursor_created_at, cursor_id_db) = if let Some(cursor_id) = cursor {
            let cursor_row = MessageSql::select_cursor_row(
                self.pool.as_ref(),
                cursor_id.into_db(),
                conversation_id_db.clone(),
                user_id_db.clone(),
            )
                .await
                .map_err(|e| e.to_repository_error())?
                .into_iter()
                .next()
                .ok_or(RepositoryError::NotFound)?;

            (Some(cursor_row.created_at), Some(cursor_row.id))
        } else {
            (None, None)
        };
        let records = MessageSql::select_message_page(
            self.pool.as_ref(),
            conversation_id_db,
            user_id_db,
            cursor_created_at,
            cursor_id_db,
            query_limit,
        )
        .await
        .map_err(|e| e.to_repository_error())?;

        let has_more = records.len() > page_size;
        let records = if has_more {
            records.into_iter().take(page_size).collect::<Vec<_>>()
        } else {
            records
        };
        let next_cursor = if has_more {
            records.last().map(|record| record.id.into_domain())
        } else {
            None
        };

        let mut messages = records.iter().map(Message::from).collect::<Vec<_>>();
        messages.sort_by(|a, b| {
            a.created_at
                .cmp(&b.created_at)
                .then_with(|| a.id.cmp(&b.id))
        });

        Ok(MessagePage {
            messages,
            next_cursor,
        })
    }

    async fn update_message(&self, message: &Message) -> Result<Message, RepositoryError> {
        let existing = database::models::message::Message::select_by_map(
            self.pool.as_ref(),
            value! { "id": message.id.into_db(), "user_id": message.user_id.into_db() },
        )
        .await
        .map_err(|e| e.to_repository_error())?;

        if existing.is_empty() {
            return Err(RepositoryError::NotFound);
        }

        let updated_message_db = database::models::message::Message::from(message);

        MessageSql::update_message(self.pool.as_ref(), &updated_message_db)
            .await
            .map_err(|e| e.to_repository_error())?;

        Ok(message.clone())
    }
}
