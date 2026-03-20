use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, Responder, web};
use application::{
    commands::{
        ProcessTelnyxWebhookCommand, TelnyxWebhookMessageError, TelnyxWebhookMessageParticipant,
        TelnyxWebhookMessagePayload,
    },
    usecases::{
        UsecaseError,
        process_telnyx_messaging_webhook_usecase::{
            MessagingWebhookNotificationKind, ProcessTelnyxMessagingWebhookUsecase,
        },
    },
};
use domain::repositories::{
    conversation_repository::ConversationRepository, message_repository::MessageRepository,
    phone_number_repository::PhoneNumberRepository,
    processed_webhook_event_repository::ProcessedWebhookEventRepository,
};
use serde::Serialize;
use telnyx::verify_messaging_webhook;
use time::OffsetDateTime;
use tracing::{error, warn};

use crate::{
    dto::{ConversationProps, MessageEventProps, MessageProps},
    realtime::MessageEventBroadcaster,
    webhook_forwarding::TelnyxWebhookForwarder,
};

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

pub async fn handle_telnyx_messaging_webhook(
    req: HttpRequest,
    body: web::Bytes,
    telnyx_public_key: web::Data<String>,
    conversation_repository: web::Data<Arc<dyn ConversationRepository>>,
    message_repository: web::Data<Arc<dyn MessageRepository>>,
    phone_number_repository: web::Data<Arc<dyn PhoneNumberRepository>>,
    processed_webhook_event_repository: web::Data<Arc<dyn ProcessedWebhookEventRepository>>,
    message_event_broadcaster: web::Data<Arc<MessageEventBroadcaster>>,
    webhook_forwarder: web::Data<TelnyxWebhookForwarder>,
) -> impl Responder {
    let signature_header = req
        .headers()
        .get("telnyx-signature-ed25519")
        .and_then(|value| value.to_str().ok());
    let timestamp_header = req
        .headers()
        .get("telnyx-timestamp")
        .and_then(|value| value.to_str().ok());

    let webhook = match verify_messaging_webhook(
        body.as_ref(),
        signature_header,
        timestamp_header,
        telnyx_public_key.get_ref(),
        OffsetDateTime::now_utc(),
    ) {
        Ok(webhook) => webhook,
        Err(err) => {
            warn!("rejected Telnyx webhook: {}", err);
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid Telnyx webhook".to_owned(),
            });
        }
    };

    webhook_forwarder.forward_verified_webhook(
        body.to_vec(),
        collect_forwarded_headers(req.headers()),
        webhook.data.id.clone(),
        webhook.data.event_type.clone(),
    );

    let raw_payload = match serde_json::from_slice::<serde_json::Value>(body.as_ref()) {
        Ok(raw_payload) => raw_payload,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid Telnyx webhook".to_owned(),
            });
        }
    };

    let cmd = ProcessTelnyxWebhookCommand {
        event_id: webhook.data.id,
        event_type: webhook.data.event_type,
        occurred_at: webhook.data.occurred_at,
        payload: map_payload(webhook.data.payload),
        raw_payload,
    };
    let usecase = ProcessTelnyxMessagingWebhookUsecase::builder()
        .conversation_repository(conversation_repository.get_ref().clone())
        .message_repository(message_repository.get_ref().clone())
        .phone_number_repository(phone_number_repository.get_ref().clone())
        .processed_webhook_event_repository(processed_webhook_event_repository.get_ref().clone())
        .build();

    match usecase.execute(cmd).await {
        Ok(result) => {
            if let Some(notification) = result.notification {
                let event_type = match notification.kind {
                    MessagingWebhookNotificationKind::MessageCreated => "message.created",
                    MessagingWebhookNotificationKind::MessageUpdated => "message.updated",
                };

                message_event_broadcaster.publish(
                    notification.user_id,
                    MessageEventProps {
                        event_type: event_type.to_owned(),
                        message: MessageProps::from(&notification.message),
                        conversation: ConversationProps::from(&notification.conversation),
                    },
                );
            }

            HttpResponse::Ok().finish()
        }
        Err(err) => {
            log_webhook_error(&err);

            match err {
                UsecaseError::Validation(_) | UsecaseError::EntityNotFound => {
                    HttpResponse::BadRequest().json(ErrorResponse {
                        error: "Invalid Telnyx webhook".to_owned(),
                    })
                }
                _ => HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Unable to process Telnyx webhook right now.".to_owned(),
                }),
            }
        }
    }
}

fn map_payload(payload: telnyx::TelnyxMessagingWebhookPayload) -> TelnyxWebhookMessagePayload {
    TelnyxWebhookMessagePayload {
        provider_message_id: payload.id,
        from_phone_number: payload.from.and_then(|value| value.phone_number),
        to: payload
            .to
            .into_iter()
            .map(|participant| TelnyxWebhookMessageParticipant {
                phone_number: participant.phone_number,
                status: participant.status,
            })
            .collect(),
        text: payload.text,
        received_at: payload.received_at,
        sent_at: payload.sent_at,
        completed_at: payload.completed_at,
        errors: payload
            .errors
            .into_iter()
            .map(|error| TelnyxWebhookMessageError {
                code: error.code,
                detail: error.detail,
                title: error.title,
            })
            .collect(),
    }
}

fn log_webhook_error(err: &UsecaseError) {
    match err {
        UsecaseError::Validation(_) | UsecaseError::EntityNotFound => {
            warn!("ignored invalid Telnyx webhook payload: {}", err);
        }
        _ => {
            error!("failed to process Telnyx webhook: {}", err);
        }
    }
}

fn collect_forwarded_headers(
    headers: &actix_web::http::header::HeaderMap,
) -> Vec<(String, String)> {
    headers
        .iter()
        .filter_map(|(name, value)| {
            value
                .to_str()
                .ok()
                .map(|value| (name.as_str().to_owned(), value.to_owned()))
        })
        .collect()
}
