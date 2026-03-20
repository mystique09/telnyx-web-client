use std::{sync::Arc, time::Duration};

use actix_session::Session;
use actix_web::{HttpResponse, Responder, http::header, web, web::Bytes};
use futures_util::stream::unfold;
use tokio::{
    sync::broadcast::error::RecvError,
    time::{Instant, interval_at},
};
use tracing::error;

use crate::{realtime::MessageEventBroadcaster, session::session_user_id};

pub async fn stream_message_events(
    session: Session,
    message_event_broadcaster: web::Data<Arc<MessageEventBroadcaster>>,
) -> impl Responder {
    let Some(user_id) = session_user_id(&session) else {
        return HttpResponse::Unauthorized().finish();
    };

    let receiver = message_event_broadcaster.subscribe(user_id);
    let keepalive = interval_at(
        Instant::now() + Duration::from_secs(25),
        Duration::from_secs(25),
    );
    let stream = unfold(
        (receiver, keepalive, true),
        move |(mut receiver, mut keepalive, first_frame)| async move {
            if first_frame {
                return Some((
                    Ok::<Bytes, actix_web::Error>(Bytes::from_static(b"retry: 5000\n\n")),
                    (receiver, keepalive, false),
                ));
            }

            loop {
                tokio::select! {
                    result = receiver.recv() => {
                        match result {
                            Ok(event) => {
                                match serde_json::to_string(&event) {
                                    Ok(json) => {
                                        let frame = format!("event: {}\ndata: {}\n\n", event.event_type, json);
                                        return Some((Ok(Bytes::from(frame)), (receiver, keepalive, false)));
                                    }
                                    Err(err) => {
                                        error!("failed to serialize SSE payload: {}", err);
                                        continue;
                                    }
                                }
                            }
                            Err(RecvError::Lagged(_)) => continue,
                            Err(RecvError::Closed) => return None,
                        }
                    }
                    _ = keepalive.tick() => {
                        return Some((Ok(Bytes::from_static(b": keepalive\n\n")), (receiver, keepalive, false)));
                    }
                }
            }
        },
    );

    HttpResponse::Ok()
        .insert_header((header::CONTENT_TYPE, "text/event-stream; charset=utf-8"))
        .insert_header((header::CACHE_CONTROL, "no-cache, no-transform"))
        .insert_header((header::CONTENT_ENCODING, "identity"))
        .insert_header(("Connection", "keep-alive"))
        .insert_header(("X-Accel-Buffering", "no"))
        .streaming(stream)
}
