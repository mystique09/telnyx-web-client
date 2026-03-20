use async_trait::async_trait;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use domain::traits::outbound_message_service::{
    OutboundMessageError, OutboundMessageService, SendMessageRequest, SendMessageResponse,
};
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, bon::Builder)]
pub struct TelnyxClient {
    api_key: String,
    base_url: String,
    messaging_profile_id: String,
    #[builder(default = Client::new())]
    http_client: Client,
}

#[derive(Debug, Serialize)]
struct SendTelnyxMessageRequest<'a> {
    messaging_profile_id: &'a str,
    from: &'a str,
    to: &'a str,
    text: &'a str,
}

#[derive(Debug, Deserialize)]
struct SendTelnyxMessageResponse {
    data: SendTelnyxMessageResponseData,
}

#[derive(Debug, Deserialize)]
struct SendTelnyxMessageResponseData {
    id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TelnyxMessagingWebhook {
    pub data: TelnyxMessagingWebhookData,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TelnyxMessagingWebhookData {
    pub event_type: String,
    pub id: String,
    #[serde(with = "time::serde::iso8601")]
    pub occurred_at: OffsetDateTime,
    pub payload: TelnyxMessagingWebhookPayload,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TelnyxMessagingWebhookPayload {
    pub id: String,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub from: Option<TelnyxMessagingWebhookEndpoint>,
    #[serde(default)]
    pub to: Vec<TelnyxMessagingWebhookEndpoint>,
    #[serde(default, with = "time::serde::iso8601::option")]
    pub received_at: Option<OffsetDateTime>,
    #[serde(default, with = "time::serde::iso8601::option")]
    pub sent_at: Option<OffsetDateTime>,
    #[serde(default, with = "time::serde::iso8601::option")]
    pub completed_at: Option<OffsetDateTime>,
    #[serde(default)]
    pub errors: Vec<TelnyxErrorItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TelnyxMessagingWebhookEndpoint {
    #[serde(default)]
    pub phone_number: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct TelnyxErrorEnvelope {
    #[serde(default)]
    errors: Vec<TelnyxErrorItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TelnyxErrorItem {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub detail: Option<String>,
    #[serde(default)]
    pub code: Option<String>,
}

#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
pub enum WebhookVerificationError {
    #[error("Missing telnyx-signature-ed25519 header")]
    MissingSignature,
    #[error("Missing telnyx-timestamp header")]
    MissingTimestamp,
    #[error("Invalid telnyx-timestamp header")]
    InvalidTimestamp,
    #[error("Webhook timestamp is outside the allowed tolerance")]
    StaleTimestamp,
    #[error("Invalid Telnyx public key")]
    InvalidPublicKey,
    #[error("Invalid Telnyx signature encoding")]
    InvalidSignatureEncoding,
    #[error("Webhook body is not valid UTF-8")]
    InvalidUtf8,
    #[error("Webhook signature verification failed")]
    InvalidSignature,
    #[error("Webhook body is not valid JSON")]
    InvalidJson,
}

impl TelnyxClient {
    fn messages_url(&self) -> String {
        format!("{}/v2/messages", self.base_url.trim_end_matches('/'))
    }
}

#[async_trait]
impl OutboundMessageService for TelnyxClient {
    async fn send_text_message(
        &self,
        request: SendMessageRequest,
    ) -> Result<SendMessageResponse, OutboundMessageError> {
        let response = self
            .http_client
            .post(self.messages_url())
            .bearer_auth(&self.api_key)
            .json(&SendTelnyxMessageRequest {
                messaging_profile_id: &self.messaging_profile_id,
                from: &request.from,
                to: &request.to,
                text: &request.text,
            })
            .send()
            .await
            .map_err(|err| OutboundMessageError::Unavailable(err.to_string()))?;

        if response.status().is_success() {
            let body = response
                .json::<SendTelnyxMessageResponse>()
                .await
                .map_err(|err| OutboundMessageError::Unavailable(err.to_string()))?;

            return Ok(SendMessageResponse {
                provider_message_id: body.data.id,
            });
        }

        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        let message = telnyx_error_message(status, &body);

        if matches!(status, StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN)
            || status.is_server_error()
        {
            return Err(OutboundMessageError::Unavailable(message));
        }

        if status.is_client_error() {
            return Err(OutboundMessageError::Rejected(message));
        }

        Err(OutboundMessageError::Unavailable(message))
    }
}

pub fn verify_messaging_webhook(
    raw_body: &[u8],
    signature_header: Option<&str>,
    timestamp_header: Option<&str>,
    public_key: &str,
    now: OffsetDateTime,
) -> Result<TelnyxMessagingWebhook, WebhookVerificationError> {
    let signature_header = signature_header
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or(WebhookVerificationError::MissingSignature)?;
    let timestamp_header = timestamp_header
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or(WebhookVerificationError::MissingTimestamp)?;

    let timestamp = timestamp_header
        .parse::<i64>()
        .map_err(|_| WebhookVerificationError::InvalidTimestamp)?;
    if (now.unix_timestamp() - timestamp).abs() > 300 {
        return Err(WebhookVerificationError::StaleTimestamp);
    }

    let public_key_bytes = parse_public_key(public_key)?;
    let verifying_key = VerifyingKey::from_bytes(&public_key_bytes)
        .map_err(|_| WebhookVerificationError::InvalidPublicKey)?;
    let signature_bytes = STANDARD
        .decode(signature_header)
        .map_err(|_| WebhookVerificationError::InvalidSignatureEncoding)?;
    let signature = Signature::from_slice(&signature_bytes)
        .map_err(|_| WebhookVerificationError::InvalidSignatureEncoding)?;
    let payload =
        std::str::from_utf8(raw_body).map_err(|_| WebhookVerificationError::InvalidUtf8)?;
    let signed_message = format!("{timestamp_header}|{payload}");

    verifying_key
        .verify(signed_message.as_bytes(), &signature)
        .map_err(|_| WebhookVerificationError::InvalidSignature)?;

    serde_json::from_slice::<TelnyxMessagingWebhook>(raw_body)
        .map_err(|_| WebhookVerificationError::InvalidJson)
}

fn parse_public_key(public_key: &str) -> Result<[u8; 32], WebhookVerificationError> {
    let trimmed = public_key.trim();
    let key_material = if trimmed.contains("BEGIN") {
        trimmed
            .lines()
            .filter(|line| !line.starts_with("-----"))
            .collect::<String>()
    } else {
        trimmed.to_owned()
    };

    let decoded = if is_hex(&key_material) {
        decode_hex(&key_material)?
    } else {
        STANDARD
            .decode(key_material)
            .map_err(|_| WebhookVerificationError::InvalidPublicKey)?
    };

    decoded
        .try_into()
        .map_err(|_| WebhookVerificationError::InvalidPublicKey)
}

fn is_hex(value: &str) -> bool {
    value.len() % 2 == 0 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn decode_hex(value: &str) -> Result<Vec<u8>, WebhookVerificationError> {
    let mut bytes = Vec::with_capacity(value.len() / 2);
    let mut chars = value.as_bytes().chunks_exact(2);

    for chunk in &mut chars {
        let pair =
            std::str::from_utf8(chunk).map_err(|_| WebhookVerificationError::InvalidPublicKey)?;
        let byte =
            u8::from_str_radix(pair, 16).map_err(|_| WebhookVerificationError::InvalidPublicKey)?;
        bytes.push(byte);
    }

    if !chars.remainder().is_empty() {
        return Err(WebhookVerificationError::InvalidPublicKey);
    }

    Ok(bytes)
}

fn telnyx_error_message(status: StatusCode, body: &str) -> String {
    if let Ok(parsed) = serde_json::from_str::<TelnyxErrorEnvelope>(body) {
        let details = parsed
            .errors
            .into_iter()
            .map(|item| {
                item.detail
                    .or(item.title)
                    .or(item.code)
                    .unwrap_or_else(|| status.to_string())
            })
            .collect::<Vec<_>>();

        if !details.is_empty() {
            return details.join(", ");
        }
    }

    if body.trim().is_empty() {
        status.to_string()
    } else {
        body.trim().to_owned()
    }
}

#[cfg(test)]
mod tests {
    use base64::{Engine as _, engine::general_purpose::STANDARD};
    use domain::traits::outbound_message_service::{
        OutboundMessageError, OutboundMessageService, SendMessageRequest,
    };
    use ed25519_dalek::{Signer, SigningKey};
    use time::OffsetDateTime;
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{body_json, header, method, path},
    };

    use super::{TelnyxClient, WebhookVerificationError, verify_messaging_webhook};

    #[tokio::test]
    async fn sends_expected_request_and_parses_provider_message_id() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v2/messages"))
            .and(header("authorization", "Bearer test-api-key"))
            .and(body_json(serde_json::json!({
                "messaging_profile_id": "test-messaging-profile-id",
                "from": "+13125550100",
                "to": "+14155551234",
                "text": "Hello"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "data": {
                    "id": "provider-message-id"
                }
            })))
            .expect(1)
            .mount(&server)
            .await;

        let client = TelnyxClient::builder()
            .api_key("test-api-key".to_owned())
            .base_url(server.uri())
            .messaging_profile_id("test-messaging-profile-id".to_owned())
            .build();

        let response = client
            .send_text_message(SendMessageRequest {
                from: "+13125550100".to_owned(),
                to: "+14155551234".to_owned(),
                text: "Hello".to_owned(),
            })
            .await
            .expect("request should succeed");

        assert_eq!(response.provider_message_id, "provider-message-id");
    }

    #[tokio::test]
    async fn maps_provider_validation_errors_to_rejected() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v2/messages"))
            .respond_with(ResponseTemplate::new(422).set_body_json(serde_json::json!({
                "errors": [
                    { "detail": "Invalid to number format" }
                ]
            })))
            .mount(&server)
            .await;

        let client = TelnyxClient::builder()
            .api_key("test-api-key".to_owned())
            .base_url(server.uri())
            .messaging_profile_id("test-messaging-profile-id".to_owned())
            .build();

        let err = client
            .send_text_message(SendMessageRequest {
                from: "+13125550100".to_owned(),
                to: "bad-number".to_owned(),
                text: "Hello".to_owned(),
            })
            .await
            .expect_err("request should fail");

        assert!(matches!(
            err,
            OutboundMessageError::Rejected(message) if message == "Invalid to number format"
        ));
    }

    #[tokio::test]
    async fn maps_auth_failures_to_unavailable() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v2/messages"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "errors": [
                    { "detail": "Authentication failed" }
                ]
            })))
            .mount(&server)
            .await;

        let client = TelnyxClient::builder()
            .api_key("bad-key".to_owned())
            .base_url(server.uri())
            .messaging_profile_id("test-messaging-profile-id".to_owned())
            .build();

        let err = client
            .send_text_message(SendMessageRequest {
                from: "+13125550100".to_owned(),
                to: "+14155551234".to_owned(),
                text: "Hello".to_owned(),
            })
            .await
            .expect_err("request should fail");

        assert!(matches!(
            err,
            OutboundMessageError::Unavailable(message) if message == "Authentication failed"
        ));
    }

    #[test]
    fn verifies_valid_webhook_signature_and_parses_payload() {
        let signing_key = SigningKey::from_bytes(&[7_u8; 32]);
        let public_key = STANDARD.encode(signing_key.verifying_key().as_bytes());
        let timestamp = OffsetDateTime::now_utc().unix_timestamp().to_string();
        let payload = r#"{"data":{"event_type":"message.sent","id":"event-1","occurred_at":"2024-01-15T21:32:13.596+00:00","payload":{"id":"provider-message-id","text":"Hello","from":{"phone_number":"+13125550100"},"to":[{"phone_number":"+14155551234","status":"sent"}],"errors":[]},"record_type":"event"}}"#;
        let signed_payload = format!("{timestamp}|{payload}");
        let signature = signing_key.sign(signed_payload.as_bytes());
        let signature_header = STANDARD.encode(signature.to_bytes());

        let event = verify_messaging_webhook(
            payload.as_bytes(),
            Some(&signature_header),
            Some(&timestamp),
            &public_key,
            OffsetDateTime::now_utc(),
        )
        .expect("signature should verify");

        assert_eq!(event.data.event_type, "message.sent");
        assert_eq!(event.data.payload.id, "provider-message-id");
    }

    #[test]
    fn rejects_missing_signature_header() {
        let err = verify_messaging_webhook(
            b"{}",
            None,
            Some("1710000000"),
            "invalid",
            OffsetDateTime::now_utc(),
        )
        .expect_err("verification should fail");

        assert_eq!(err, WebhookVerificationError::MissingSignature);
    }

    #[test]
    fn rejects_invalid_signature() {
        let signing_key = SigningKey::from_bytes(&[7_u8; 32]);
        let public_key = STANDARD.encode(signing_key.verifying_key().as_bytes());
        let timestamp = OffsetDateTime::now_utc().unix_timestamp().to_string();
        let payload = r#"{"data":{"event_type":"message.sent","id":"event-1","occurred_at":"2024-01-15T21:32:13.596+00:00","payload":{"id":"provider-message-id","errors":[],"to":[{"phone_number":"+14155551234","status":"sent"}]},"record_type":"event"}}"#;

        let err = verify_messaging_webhook(
            payload.as_bytes(),
            Some("ZmFrZS1zaWduYXR1cmU="),
            Some(&timestamp),
            &public_key,
            OffsetDateTime::now_utc(),
        )
        .expect_err("verification should fail");

        assert_eq!(err, WebhookVerificationError::InvalidSignatureEncoding);
    }

    #[test]
    fn rejects_stale_timestamp() {
        let signing_key = SigningKey::from_bytes(&[7_u8; 32]);
        let public_key = STANDARD.encode(signing_key.verifying_key().as_bytes());
        let now = OffsetDateTime::now_utc();
        let timestamp = (now.unix_timestamp() - 301).to_string();
        let payload = b"{\"data\":{\"event_type\":\"message.sent\",\"id\":\"event-1\",\"occurred_at\":\"2024-01-15T21:32:13.596+00:00\",\"payload\":{\"id\":\"provider-message-id\",\"errors\":[],\"to\":[{\"phone_number\":\"+14155551234\",\"status\":\"sent\"}]},\"record_type\":\"event\"}}";
        let signed_payload = format!(
            "{}|{}",
            timestamp,
            std::str::from_utf8(payload).expect("utf8")
        );
        let signature = signing_key.sign(signed_payload.as_bytes());
        let signature_header = STANDARD.encode(signature.to_bytes());

        let err = verify_messaging_webhook(
            payload,
            Some(&signature_header),
            Some(&timestamp),
            &public_key,
            now,
        )
        .expect_err("verification should fail");

        assert_eq!(err, WebhookVerificationError::StaleTimestamp);
    }
}
