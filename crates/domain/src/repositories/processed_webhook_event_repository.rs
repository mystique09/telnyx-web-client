use async_trait::async_trait;

use crate::{
    models::processed_webhook_event::ProcessedWebhookEvent,
    repositories::RepositoryError,
};

#[async_trait]
pub trait ProcessedWebhookEventRepository: Send + Sync + 'static {
    async fn create_processed_webhook_event(
        &self,
        event: &ProcessedWebhookEvent,
    ) -> Result<(), RepositoryError>;

    async fn find_by_event_id(
        &self,
        event_id: &str,
    ) -> Result<ProcessedWebhookEvent, RepositoryError>;
}
