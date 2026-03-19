use async_trait::async_trait;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SendMessageRequest {
    pub from: String,
    pub to: String,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SendMessageResponse {
    pub provider_message_id: String,
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum OutboundMessageError {
    #[error("Message rejected: {0}")]
    Rejected(String),
    #[error("Outbound messaging unavailable: {0}")]
    Unavailable(String),
}

#[async_trait]
pub trait OutboundMessageService: Send + Sync + 'static {
    async fn send_text_message(
        &self,
        request: SendMessageRequest,
    ) -> Result<SendMessageResponse, OutboundMessageError>;
}
