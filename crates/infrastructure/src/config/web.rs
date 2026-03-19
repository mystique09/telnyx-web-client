use std::net::Ipv4Addr;

use crate::config::ConfigError;

#[derive(Debug, bon::Builder)]
pub struct WebConfig {
    pub host: Ipv4Addr,
    pub port: u16,
    pub paseto_symmetric_key: String,
    pub session_secret: String,
    pub telnyx_api_key: String,
    pub telnyx_messaging_profile_id: String,
    pub telnyx_api_base_url: String,
    pub telnyx_public_key: String,
    pub telnyx_webhook_forward_urls: Vec<String>,
}

impl WebConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let host = std::env::var("HOST")
            .map_err(|_| ConfigError::EnvVarNotFound("HOST".to_string()))?
            .parse::<Ipv4Addr>()
            .map_err(|_| ConfigError::EnvVarNotValid("HOST".to_string()))?;

        let port = std::env::var("PORT")
            .map_err(|_| ConfigError::EnvVarNotFound("PORT".to_string()))?
            .parse::<u16>()
            .map_err(|_| ConfigError::EnvVarNotValid("PORT".to_string()))?;

        let paseto_symmetric_key = std::env::var("PASETO_SEMETRIC_KEY")
            .map_err(|_| ConfigError::EnvVarNotFound("PASETO_SEMETRIC_KEY".to_string()))?;

        let session_secret = std::env::var("SESSION_SECRET")
            .map_err(|_| ConfigError::EnvVarNotFound("SESSION_SECRET".to_string()))?;
        let telnyx_api_key = std::env::var("TELNYX_API_KEY")
            .map_err(|_| ConfigError::EnvVarNotFound("TELNYX_API_KEY".to_string()))?;
        let telnyx_messaging_profile_id = std::env::var("TELNYX_MESSAGING_PROFILE_ID")
            .map_err(|_| ConfigError::EnvVarNotFound("TELNYX_MESSAGING_PROFILE_ID".to_string()))?;
        let telnyx_api_base_url = std::env::var("TELNYX_API_BASE_URL")
            .unwrap_or_else(|_| "https://api.telnyx.com".to_string());
        let telnyx_public_key = std::env::var("TELNYX_PUBLIC_KEY")
            .map_err(|_| ConfigError::EnvVarNotFound("TELNYX_PUBLIC_KEY".to_string()))?;
        let telnyx_webhook_forward_urls = std::env::var("TELNYX_WEBHOOK_FORWARD_URLS")
            .ok()
            .map(|value| {
                value
                    .split(',')
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                    .map(ToOwned::to_owned)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        Ok(Self::builder()
            .host(host)
            .port(port)
            .paseto_symmetric_key(paseto_symmetric_key)
            .session_secret(session_secret)
            .telnyx_api_key(telnyx_api_key)
            .telnyx_messaging_profile_id(telnyx_messaging_profile_id)
            .telnyx_api_base_url(telnyx_api_base_url)
            .telnyx_public_key(telnyx_public_key)
            .telnyx_webhook_forward_urls(telnyx_webhook_forward_urls)
            .build())
    }

    pub fn addrs(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
