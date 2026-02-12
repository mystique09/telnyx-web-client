use async_trait::async_trait;

use crate::repositories::RepositoryError;

#[async_trait]
pub trait MessageRepository: Send + Sync + 'static {
    async fn count_by_user_id(&self, user_id: &uuid::Uuid) -> Result<u64, RepositoryError>;
}
