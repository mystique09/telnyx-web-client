use std::sync::Arc;

use domain::repositories::message_repository::MessageRepository;
use domain::repositories::RepositoryError;

use rbatis::{RBatis, async_trait};
use rbs::value;

use crate::database;
use crate::database::models::UuidExt;
use crate::repositories::RbsErrorExt;

#[derive(Debug, bon::Builder)]
pub struct MessageRepositoryImpl {
    pool: Arc<RBatis>,
}

#[async_trait]
impl MessageRepository for MessageRepositoryImpl {
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
}
