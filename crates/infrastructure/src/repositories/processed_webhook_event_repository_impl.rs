use std::sync::Arc;

use domain::{
    models::processed_webhook_event::ProcessedWebhookEvent,
    repositories::{
        RepositoryError,
        processed_webhook_event_repository::ProcessedWebhookEventRepository,
    },
};
use rbatis::{RBatis, async_trait};
use rbs::value;

use crate::{
    database,
    repositories::RbsErrorExt,
};

#[derive(Debug, bon::Builder)]
pub struct ProcessedWebhookEventRepositoryImpl {
    pool: Arc<RBatis>,
}

#[async_trait]
impl ProcessedWebhookEventRepository for ProcessedWebhookEventRepositoryImpl {
    async fn create_processed_webhook_event(
        &self,
        event: &ProcessedWebhookEvent,
    ) -> Result<(), RepositoryError> {
        let record = database::models::processed_webhook_event::ProcessedWebhookEvent::from(event);

        database::models::processed_webhook_event::ProcessedWebhookEvent::insert(
            self.pool.as_ref(),
            &record,
        )
        .await
        .map_err(|e| e.to_repository_error())?;

        Ok(())
    }

    async fn find_by_event_id(
        &self,
        event_id: &str,
    ) -> Result<ProcessedWebhookEvent, RepositoryError> {
        let record = database::models::processed_webhook_event::ProcessedWebhookEvent::select_by_map(
            self.pool.as_ref(),
            value! { "event_id": event_id },
        )
        .await
        .map_err(|e| e.to_repository_error())?
        .into_iter()
        .next()
        .ok_or(RepositoryError::NotFound)?;

        Ok(ProcessedWebhookEvent::from(&record))
    }
}
