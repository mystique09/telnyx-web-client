use rbatis::rbdc::{DateTime, types::Json};
use serde::{Deserialize, Serialize};

use crate::database::models::{datetime_to_offset_datetime, offset_datetime_to_datetime};

#[derive(Debug, bon::Builder, Serialize, Deserialize)]
pub struct ProcessedWebhookEvent {
    pub event_id: String,
    pub event_type: String,
    pub provider_message_id: Option<String>,
    pub occurred_at: DateTime,
    pub payload_json: Json,
    pub created_at: DateTime,
}

rbatis::crud!(ProcessedWebhookEvent {}, "processed_webhook_events");

impl From<&ProcessedWebhookEvent>
    for domain::models::processed_webhook_event::ProcessedWebhookEvent
{
    fn from(value: &ProcessedWebhookEvent) -> Self {
        Self::builder()
            .event_id(value.event_id.to_owned())
            .event_type(value.event_type.to_owned())
            .maybe_provider_message_id(value.provider_message_id.to_owned())
            .occurred_at(datetime_to_offset_datetime(value.occurred_at.to_owned()))
            .payload_json(value.payload_json.clone().into())
            .created_at(datetime_to_offset_datetime(value.created_at.to_owned()))
            .build()
    }
}

impl From<&domain::models::processed_webhook_event::ProcessedWebhookEvent>
    for ProcessedWebhookEvent
{
    fn from(value: &domain::models::processed_webhook_event::ProcessedWebhookEvent) -> Self {
        Self::builder()
            .event_id(value.event_id.to_owned())
            .event_type(value.event_type.to_owned())
            .maybe_provider_message_id(value.provider_message_id.to_owned())
            .occurred_at(offset_datetime_to_datetime(value.occurred_at))
            .payload_json(value.payload_json.to_owned().into())
            .created_at(offset_datetime_to_datetime(value.created_at))
            .build()
    }
}
