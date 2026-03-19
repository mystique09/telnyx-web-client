use time::OffsetDateTime;

#[derive(Debug, Clone, bon::Builder)]
pub struct ProcessedWebhookEvent {
    pub event_id: String,
    pub event_type: String,
    pub provider_message_id: Option<String>,
    pub occurred_at: OffsetDateTime,
    pub payload_json: serde_json::Value,
    pub created_at: OffsetDateTime,
}
