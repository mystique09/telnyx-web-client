use std::net::Ipv4Addr;

use crate::config::ConfigError;

#[derive(Debug, bon::Builder)]
pub struct WebConfig {
    pub host: Ipv4Addr,
    pub port: u16,
    pub paseto_symmetric_key: String,
    pub session_secret: String,
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

        Ok(Self::builder()
            .host(host)
            .port(port)
            .paseto_symmetric_key(paseto_symmetric_key)
            .session_secret(session_secret)
            .build())
    }

    pub fn addrs(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
