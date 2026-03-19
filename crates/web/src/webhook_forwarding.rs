use std::time::Duration;

use actix_web::rt;
use tracing::warn;

#[derive(Debug, Clone)]
pub struct TelnyxWebhookForwarder {
    forward_urls: Vec<String>,
    http_client: reqwest::Client,
}

impl TelnyxWebhookForwarder {
    pub fn new(forward_urls: Vec<String>) -> Self {
        Self {
            forward_urls,
            http_client: reqwest::Client::new(),
        }
    }

    pub fn forward_verified_webhook(
        &self,
        raw_body: Vec<u8>,
        headers: Vec<(String, String)>,
        event_id: String,
        event_type: String,
    ) {
        if self.forward_urls.is_empty() {
            return;
        }

        let forward_urls = self.forward_urls.clone();
        let http_client = self.http_client.clone();

        rt::spawn(async move {
            for forward_url in forward_urls {
                let mut request = http_client
                    .post(&forward_url)
                    .header("x-forwarded-by", "telnyx-web-client")
                    .header("x-telnyx-event-id", event_id.as_str())
                    .header("x-telnyx-event-type", event_type.as_str())
                    .timeout(Duration::from_secs(5))
                    .body(raw_body.clone());

                for (header_name, header_value) in &headers {
                    if should_skip_forwarded_header(header_name) {
                        continue;
                    }

                    request = request.header(header_name.as_str(), header_value.as_str());
                }

                match request.send().await {
                    Ok(response) if response.status().is_success() => {}
                    Ok(response) => {
                        warn!(
                            "forwarded Telnyx webhook {} ({}) to {} but received {}",
                            event_id,
                            event_type,
                            forward_url,
                            response.status()
                        );
                    }
                    Err(err) => {
                        warn!(
                            "failed to forward Telnyx webhook {} ({}) to {}: {}",
                            event_id, event_type, forward_url, err
                        );
                    }
                }
            }
        });
    }
}

fn should_skip_forwarded_header(header_name: &str) -> bool {
    matches!(
        header_name.to_ascii_lowercase().as_str(),
        "connection" | "content-length" | "host"
    )
}
